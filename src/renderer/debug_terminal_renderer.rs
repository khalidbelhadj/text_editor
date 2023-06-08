use crate::renderer::Renderer;

use termion::{
    clear, color, cursor,
    raw::{IntoRawMode, RawTerminal},
    style, terminal_size,
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
        let (width, height) = terminal_size().expect("Could not get terminal size");
        let lines = buffer.text_lines_raw();
        let lines_trimmed = lines.iter().take((height - 2) as usize).collect::<Vec<_>>();
        let gutter_offset = buffer.line_count().to_string().len().max(2) as u16 + 1;

        write!(self.stdout, "{}", cursor::Hide).unwrap();

        for i in 0..lines_trimmed.len() {
            let line_number = (i + 1) as u16;
            let line = lines_trimmed[i]
                .iter()
                .take(width as usize)
                .collect::<String>();

            // Display the line
            write!(
                self.stdout,
                "{}{}",
                cursor::Goto(gutter_offset + 1, line_number),
                line
            )
            .unwrap();

            // Clearing everything that was not overwritten
            write!(
                self.stdout,
                "{}{}",
                cursor::Goto(gutter_offset + 1 + line.len() as u16, line_number),
                clear::UntilNewline
            )
            .unwrap();

            // Clearing the gutter
            for i in 1..=gutter_offset {
                write!(self.stdout, "{} ", cursor::Goto(i as u16, line_number)).unwrap();
            }

            // Drawing the line number
            let number_offset = gutter_offset - (line_number.to_string().len()) as u16;
            write!(
                self.stdout,
                "{}{}{}",
                style::Faint,
                cursor::Goto(number_offset, line_number),
                line_number
            )
            .unwrap();
            write!(self.stdout, "{}{}", style::Reset, color::Bg(color::Reset)).unwrap();
        }

        // Clearing all lines until the status line
        for line in (lines_trimmed.len() + 1) as u16..=(height - 2) {
            write!(
                self.stdout,
                "{}{}",
                cursor::Goto(1, line),
                clear::CurrentLine
            )
            .unwrap();
        }

        self.draw_debug_info(editor);

        // Rendering cursor
        let buffer = editor.get_focused_buffer();
        let (line, column) = buffer.cursor_position();
        let gutter_offset = (buffer.line_count().to_string().len() + 1).max(3) as u16;

        write!(
            self.stdout,
            "{}",
            cursor::Goto(column as u16 + gutter_offset, line as u16)
        )
        .unwrap();
        write!(self.stdout, "{}", cursor::Show).unwrap();
        write!(self.stdout, "{}", cursor::BlinkingBlock).unwrap();

        self.stdout.flush().unwrap();
    }

    fn render_cursor(&mut self, editor: &Editor) {
        self.render(editor);
    }

    fn render_status_line(&mut self, editor: &Editor) {}

    fn render_minibuffer_prompt(&mut self, editor: &Editor, message: &str) {}

    fn clear_minibuffer(&mut self, editor: &Editor) {}
}

impl DebugTerminalRenderer {
    fn draw_debug_info(&mut self, editor: &Editor) {
        let buffer = editor.get_focused_buffer();
        let text = &buffer.data;

        let (width, height) = terminal_size().unwrap();
        let (line, column) = buffer.cursor_position();

        let mut t = height - 8;
        let mut iota = || {
            t += 1;
            t
        };
        write!(
            self.stdout,
            "{}---------- DEBUG INFO ---------- ",
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
            "{}text len: {}",
            cursor::Goto(1 as u16, iota()),
            text.len()
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
