use crate::renderer::Renderer;

use termion::{
    clear, color, cursor,
    raw::{IntoRawMode, RawTerminal},
    terminal_size,
};

use std::io::{stdout, Stdout, Write};

use crate::editor::Editor;

pub struct DebugTerminalRenderer {
    stdout: RawTerminal<Stdout>,
}

impl Renderer for DebugTerminalRenderer {
    fn new() -> Self {
        DebugTerminalRenderer {
            stdout: stdout().into_raw_mode().unwrap(),
        }
    }

    fn render(&mut self, editor: &Editor) {
        let buffer = editor.get_focused_buffer();
        let text = &buffer.data;

        let (width, height) = terminal_size().unwrap();
        let (line, column) = buffer.cursor_position();

        let mut line_number: u16 = 1;
        let mut line_offset = 0;

        write!(self.stdout, "{}", cursor::Goto(1, 1)).unwrap();
        write!(self.stdout, "{}", cursor::Hide).unwrap();
        write!(self.stdout, "{}", clear::All).unwrap();

        // Main loop for rendering text
        for i in 0..text.len() {
            // TODO: Make sure column is less than width
            if line_number > height - 1 {
                break;
            }

            if text[i] == '\n' {
                line_number += 1;
                line_offset = i + 1;
                continue;
            }

            if buffer.gap_start <= i && i < buffer.gap_len + buffer.gap_start {
                // Display the actual character
                write!(
                    self.stdout,
                    "{}{}",
                    cursor::Goto((i - line_offset + 1) as u16, line_number),
                    '_'
                )
                .unwrap();
                continue;
            }

            // Display the actual character
            write!(
                self.stdout,
                "{}{}",
                cursor::Goto((i - line_offset + 1) as u16, line_number),
                text[i]
            )
            .unwrap();

            // use std::{thread, time};
            // let ten_millis = time::Duration::from_millis(1000);
            // thread::sleep(ten_millis);
            // self.stdout.flush().unwrap();
        }
        self.draw_debug_info(editor, line_number);
        self.draw_status_line(width, height);
        self.draw_cursor(line as u16, column as u16);
        self.stdout.flush().unwrap();
    }

    fn render_cursor(&mut self, editor: &Editor) {
        let buffer = editor.get_focused_buffer();

        let (line, column) = buffer.cursor_position();
        let gutter_offset = (buffer.line_count().to_string().len() + 1).max(3) as u16;
        self.draw_cursor(line as u16, column as u16 + gutter_offset);
    }

    fn render_status_line(&mut self, editor: &Editor) {
        let (width, height) = terminal_size().unwrap();
        self.draw_status_line(width, height);
    }

    fn render_minibuffer_prompt(&mut self, editor: &Editor, message: &str) {
    }

    fn clear_minibuffer(&mut self, editor: &Editor) {

    }
}

impl DebugTerminalRenderer {
    fn draw_debug_info(&mut self, editor: &Editor, line_number: u16) {
        let buffer = editor.get_focused_buffer();
        let text = &buffer.data;

        let (width, height) = terminal_size().unwrap();
        let (line, column) = buffer.cursor_position();

        let mut t = line_number;
        let mut iota = || {
            t += 1;
            t
        };
        write!(
            self.stdout,
            "{}---------- DEBUG INFO ----------",
            cursor::Goto(1 as u16, iota())
        )
        .unwrap();
        write!(
            self.stdout,
            "{}cursor position: ({}, {})",
            cursor::Goto(1 as u16, iota()),
            line,
            column
        )
        .unwrap();
        write!(
            self.stdout,
            "{}width: {}, height: {}",
            cursor::Goto(1 as u16, iota()),
            width,
            height
        )
        .unwrap();
        write!(
            self.stdout,
            "{}line offsets count: {}",
            cursor::Goto(1 as u16, iota()),
            buffer.line_offsets.len()
        )
        .unwrap();
        write!(
            self.stdout,
            "{}data_len: {}",
            cursor::Goto(1 as u16, iota()),
            buffer.data.len()
        )
        .unwrap();
        write!(
            self.stdout,
            "{}gap_start: {}",
            cursor::Goto(1 as u16, iota()),
            buffer.gap_start
        )
        .unwrap();
        write!(
            self.stdout,
            "{}gap_len: {}",
            cursor::Goto(1 as u16, iota()),
            buffer.gap_len
        )
        .unwrap();
        write!(
            self.stdout,
            "{}cursor_offset: {}",
            cursor::Goto(1 as u16, iota()),
            buffer.cursor_offset
        )
        .unwrap();
        write!(
            self.stdout,
            "{}line_offsets.len(): {}",
            cursor::Goto(1 as u16, iota()),
            buffer.line_offsets.len()
        )
        .unwrap();
        write!(
            self.stdout,
            "{}line_offsets: {:?}",
            cursor::Goto(1 as u16, iota()),
            buffer.line_offsets
        )
        .unwrap();
        write!(
            self.stdout,
            "{}text len: {}",
            cursor::Goto(1 as u16, iota()),
            text.len()
        )
        .unwrap();
        write!(
            self.stdout,
            "{}last line number: {}",
            cursor::Goto(1 as u16, iota()),
            line_number
        )
        .unwrap();
    }

    pub fn draw_status_line(&mut self, width: u16, height: u16) {
        write!(
            self.stdout,
            "{}",
            cursor::Goto(1 as u16, (height - 1) as u16)
        )
        .unwrap();

        for i in 1..width as usize {
            write!(
                self.stdout,
                "{}",
                cursor::Goto(i as u16, (height - 1) as u16)
            )
            .unwrap();
            write!(self.stdout, " ",).unwrap();
        }

        write!(
            self.stdout,
            "{}{}",
            color::Bg(color::Reset),
            color::Fg(color::Reset)
        )
        .unwrap();
    }

    pub fn draw_cursor(&mut self, line: u16, column: u16) {
        // Repositioning the cursor
        write!(self.stdout, "{}", cursor::Goto(column, line)).unwrap();
        write!(self.stdout, "{}", cursor::Show).unwrap();
        write!(self.stdout, "{}", cursor::BlinkingBlock).unwrap();
    }
}
