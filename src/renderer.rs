use termion::{
    raw::{RawTerminal, IntoRawMode},
    color,
    clear,
    style,
    cursor
};

use std::io::{Write, stdout, Stdout};

use crate::editor::Editor;

pub trait Renderer {
    fn new() -> Self;
    fn render(&mut self, buffer: &Editor);
}

pub struct TerminalRenderer {
    stdout: RawTerminal<Stdout>,
    pub debug: bool
}

impl Renderer for TerminalRenderer {
    fn new() -> Self {
        TerminalRenderer {
            stdout: stdout().into_raw_mode().unwrap(),
            debug: false,
        }
    }

    fn render(&mut self, editor: &Editor) {
        write!(self.stdout, "{}", cursor::Goto(1, 1)).unwrap();

        let show_gap: bool = false;

        let view = editor.views.get(&editor.focused).unwrap(); // TODO: Change unwrap
        let buffer = editor.buffers.get(&view.buffer_id).unwrap();
        let surface = &view.surface;

        let mut text = &buffer.text();
        if self.debug && show_gap {
            text = &buffer.data;
        }

        let (width, height) = ((surface.x + surface.width), (surface.y + surface.height));
        let (line, column) = buffer.cursor_position();

        let mut line_number: u16 = 1;
        let mut line_offset = 0;

        write!(self.stdout, "{}", cursor::Hide).unwrap();
        write!(self.stdout, "{}{}", cursor::Goto(1, line_number), clear::CurrentLine).unwrap();
        for i in 0..text.len() {
            if line_number > height {
                break;
            }

            if self.debug && show_gap {
                if buffer.gap_start <= i && i < buffer.gap_start + buffer.gap_len {
                    write!(self.stdout, "{}{}", cursor::Goto((i - line_offset + 1) as u16, line_number), '_').unwrap();
                    continue;
                }
            }

            if text[i] == '\n' {
                line_number += 1;
                write!(self.stdout, "{}{}", cursor::Goto(1, line_number), clear::UntilNewline).unwrap();
                line_offset = i + 1;
                continue;
            }

            write!(self.stdout, "{}{}", cursor::Goto((i - line_offset + 1) as u16, line_number), text[i]).unwrap();
            self.stdout.flush().unwrap();

            use std::{thread, time};
            let ten_millis = time::Duration::from_millis(20);
            thread::sleep(ten_millis);
        }

        if self.debug {
            let mut t = line_number;
            let mut iota = || {t += 1; t};
            write!(self.stdout, "{}---------- DEBUG INFO ----------", cursor::Goto(1 as u16, iota())).unwrap();
            write!(self.stdout, "{}cursor position: ({}, {})", cursor::Goto(1 as u16, iota()), line, column).unwrap();
            write!(self.stdout, "{}WIDTH: {}, HEIGHT: {})", cursor::Goto(1 as u16, iota()), width, height).unwrap();
            write!(self.stdout, "{}line offsets count: {}", cursor::Goto(1 as u16, iota()), buffer.line_offsets.len()).unwrap();
            write!(self.stdout, "{}data_len: {}", cursor::Goto(1 as u16, iota()), buffer.data_len).unwrap();
            write!(self.stdout, "{}gap_start: {}", cursor::Goto(1 as u16, iota()), buffer.gap_start).unwrap();
            write!(self.stdout, "{}gap_len: {}", cursor::Goto(1 as u16, iota()), buffer.gap_len).unwrap();
            write!(self.stdout, "{}cursor_offset: {}", cursor::Goto(1 as u16, iota()), buffer.cursor_offset).unwrap();
            write!(self.stdout, "{}line_offsets.len(): {}", cursor::Goto(1 as u16, iota()), buffer.line_offsets.len()).unwrap();
            write!(self.stdout, "{}line_offsets: {:?}", cursor::Goto(1 as u16, iota()), buffer.line_offsets).unwrap();
            write!(self.stdout, "{}text len: {}", cursor::Goto(1 as u16, iota()), text.len()).unwrap();
            write!(self.stdout, "{}last line number: {}", cursor::Goto(1 as u16, iota()), line_number).unwrap();
        }

        // Drawing the status line
        write!(self.stdout, "{}", cursor::Goto(1 as u16, height as u16)).unwrap();
        write!(self.stdout, "{}", clear::CurrentLine).unwrap();

        write!(self.stdout, "{}{}COL: {} LINE: {} FILE: {:?}", color::Fg(color::Black), color::Bg(color::White), column, line, buffer.get_path_as_string()).unwrap();
        write!(self.stdout, "{}{}", color::Bg(color::Reset), color::Fg(color::Reset)).unwrap();

        write!(self.stdout, "{}", cursor::Goto(column as u16, line as u16)).unwrap();
        write!(self.stdout, "{}", cursor::Show).unwrap();
        write!(self.stdout, "{}", cursor::BlinkingBlock).unwrap();

        self.stdout.flush().unwrap();
    }
}
