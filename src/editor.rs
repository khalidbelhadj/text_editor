use std::collections::HashMap;
use crate::buffer::Buffer;
use crate::view::View;

use termion::{
    event::Key,
    raw::{RawTerminal, IntoRawMode},
    color,
    clear,
    style,
    cursor
};

use std::io::{Write, stdout, stdin, Stdout};
use std::path::PathBuf;
use std::{thread, time};

pub type ViewId = usize;
pub type BufferId = usize;

pub struct Editor {
    buffers: HashMap<BufferId, Buffer>,
    next_buffer_id: BufferId,
    views: HashMap<ViewId, View>,
    next_view_id: ViewId,
    focused: ViewId
}

impl Editor {
    pub fn new(path: Option<PathBuf>) -> Self {
        let mut editor = Editor {
            buffers: HashMap::new(),
            next_buffer_id: 0,
            views: HashMap::new(),
            next_view_id: 0,
            focused: 0
        };
        editor.open_file(path);
        editor
    }

    pub fn open_file(&mut self, path: Option<PathBuf>) {
        // TODO: Handle option from insert
        self.buffers.insert(self.next_buffer_id, Buffer::new(path));
        let (width, height) = termion::terminal_size().unwrap();
        self.views.insert(self.next_view_id, View::new(self.next_buffer_id, 0, 0, width, height));

        self.focused = self.next_view_id;

        self.next_buffer_id += 1;
        self.next_view_id += 1;
    }

    pub fn render(&self, stdout: &mut RawTerminal<Stdout>, debug: bool) {
        write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();

        let show_gap: bool = false;

        let view = self.views.get(&self.focused).unwrap(); // TODO: Change unwrap
        let buffer = self.buffers.get(&view.buffer_id).unwrap();
        let surface = &view.surface;

        let mut text = &buffer.text();
        if debug && show_gap {
            text = &buffer.data;
        }

        let (width, height) = ((surface.x + surface.width), (surface.y + surface.height));
        let (line, column) = buffer.cursor_position();

        let mut line_number: u16 = 1;
        let mut line_offset = 0;

        write!(stdout, "{}", cursor::Hide).unwrap();
        write!(stdout, "{}{}", cursor::Goto(1, line_number), clear::CurrentLine).unwrap();
        for i in 0..text.len() {
            if line_number > height {
                break;
            }

            if debug && show_gap {
                if buffer.gap_start <= i && i < buffer.gap_start + buffer.gap_len {
                    write!(stdout, "{}{}", cursor::Goto((i - line_offset + 1) as u16, line_number), '_').unwrap();
                    continue;
                }
            }

            if text[i] == '\n' {
                line_number += 1;
                line_offset = i + 1;
                write!(stdout, "{}{}", cursor::Goto(1, line_number), clear::CurrentLine).unwrap();
                continue;
            }

            write!(stdout, "{}{}", cursor::Goto((i - line_offset + 1) as u16, line_number), text[i]).unwrap();
            stdout.flush().unwrap();
        }


        if debug {
            let mut t = line_number;
            let mut iota = || {t += 1; t};
            write!(stdout, "{}---------- DEBUG INFO ----------", cursor::Goto(1 as u16, iota())).unwrap();
            write!(stdout, "{}cursor position: ({}, {})", cursor::Goto(1 as u16, iota()), line, column).unwrap();
            write!(stdout, "{}WIDTH: {}, HEIGHT: {})", cursor::Goto(1 as u16, iota()), width, height).unwrap();
            write!(stdout, "{}line offsets count: {}", cursor::Goto(1 as u16, iota()), buffer.line_offsets.len()).unwrap();
            write!(stdout, "{}data_len: {}", cursor::Goto(1 as u16, iota()), buffer.data_len).unwrap();
            write!(stdout, "{}gap_start: {}", cursor::Goto(1 as u16, iota()), buffer.gap_start).unwrap();
            write!(stdout, "{}gap_len: {}", cursor::Goto(1 as u16, iota()), buffer.gap_len).unwrap();
            write!(stdout, "{}cursor_offset: {}", cursor::Goto(1 as u16, iota()), buffer.cursor_offset).unwrap();
            write!(stdout, "{}line_offsets.len(): {}", cursor::Goto(1 as u16, iota()), buffer.line_offsets.len()).unwrap();
            write!(stdout, "{}line_offsets: {:?}", cursor::Goto(1 as u16, iota()), buffer.line_offsets).unwrap();
            write!(stdout, "{}text len: {}", cursor::Goto(1 as u16, iota()), text.len()).unwrap();
            write!(stdout, "{}last line number: {}", cursor::Goto(1 as u16, iota()), line_number).unwrap();
        }

        // Drawing the status line
        write!(stdout, "{}", cursor::Goto(1 as u16, height as u16)).unwrap();
        write!(stdout, "{}", clear::CurrentLine).unwrap();
        write!(stdout, "{}{}COL: {} LINE: {}", color::Fg(color::Black), color::Bg(color::White), column, line).unwrap();
        write!(stdout, "{}{}", color::Bg(color::Reset), color::Fg(color::Reset)).unwrap();

        write!(stdout, "{}", cursor::Goto(column as u16, line as u16)).unwrap();
        write!(stdout, "{}", cursor::Show).unwrap();
        write!(stdout, "{}", cursor::BlinkingBlock).unwrap();
    }


    // TODO: Not sure if I should be using a Result for this
    // TODO: At least use proper generics inside the Result
    pub fn handle_key(&mut self, key: Key) -> Result<(), EditorError> {
        let view = self.views.get(&self.focused).unwrap(); // TODO: Change unwrap
        let buffer = self.buffers.get_mut(&view.buffer_id).unwrap();

        let mut res: Result<(), EditorError> = Result::Ok(());
        match key {
            Key::Char(c) => {
                buffer.insert_char(c);
            },
            Key::Left => {
                buffer.move_cursor(-1);
            },
            Key::Right => {
                buffer.move_cursor(1);
            },
            Key::Backspace => {
                buffer.delete_char_backward();
            },
            Key::Ctrl(c) => {
                match c {
                    'c' => {
                        res = Err(EditorError::Quit);
                    },
                    'd' => {
                        buffer.delete_char_forward();
                    },
                    'b' => {
                        buffer.move_cursor(-1);
                    },
                    'f' => {
                        buffer.move_cursor(1);
                    },
                    'e' => {
                        buffer.move_to_eol();
                    },
                    'a' => {
                        buffer.move_to_bol();
                    },
                    'w' => {
                        buffer.save(None);
                    },
                    'p' => {
                        res = Err(EditorError::ToggleDebug);
                    },
                    'y' => {
                        self.buffers.remove(&view.buffer_id);
                        self.buffers.insert(view.buffer_id, Buffer::new(None));
                    }
                    _ => todo!("Ctrl modifier not implemented")
                }
            },
            _ => {
                todo!("key not handled key: {:?}", key);
            }
        }
        return res;
    }
}

pub enum EditorError {
    ToggleDebug,
    Quit
}
