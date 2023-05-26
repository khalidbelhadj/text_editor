use crate::{renderer::Renderer, controller::EditorState, buffer::Buffer};
use termion::{
    clear, color, cursor,
    raw::{IntoRawMode, RawTerminal},
    style, terminal_size,
};

use std::{
    fs::write,
    io::{stdout, Stdout, Write},
};
use termion::screen::{AlternateScreen, IntoAlternateScreen};

use crate::editor::Editor;

pub struct TerminalRenderer {
    // stdout: AlternateScreen<RawTerminal<Stdout>>
    stdout: RawTerminal<Stdout>,
}

impl Renderer for TerminalRenderer {
    fn new() -> Self {
        TerminalRenderer {
            stdout: stdout().into_raw_mode().unwrap(), // .into_alternate_screen()
                                                       // .unwrap()
        }
    }

    fn render(&mut self, editor: &Editor) {
        let buffer = editor.get_focused_buffer();
        let (width, height) = terminal_size().unwrap();
        let lines = buffer.text_lines();
        let lines_trimmed = lines.iter().take((height - 2)as usize).collect::<Vec<_>>();
        let gutter_offset = (buffer.line_count().to_string().len() + 2).max(3) as u16;

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
                cursor::Goto(gutter_offset, line_number),
                line
            )
            .unwrap();

            // Clear everything that wasn't overwritten in the current line
            write!(
                self.stdout,
                "{}{}",
                cursor::Goto(
                    width.min(line.len() as u16 + gutter_offset + 1) as u16,
                    line_number
                ),
                clear::UntilNewline
            )
            .unwrap();

            self.draw_line_number(line_number, gutter_offset - 1);
        }

        // Clearing all lines until the status line
        for line in (lines_trimmed.len() + 1) as u16..=(height - 2) {
            write!(self.stdout, "{}{}", cursor::Goto(1, line), clear::CurrentLine).unwrap();
        }

        self.render_status_line(&editor);
        self.render_cursor(&editor);
    }

    fn render_cursor(&mut self, editor: &Editor) {
        let buffer: &Buffer;

        let state = editor.state;

        match state {
            EditorState::Editing => {
                buffer = editor.get_focused_buffer()
            }
            EditorState::PromptResponse => {
                buffer = &editor.minibuffer;
            }
        }

        let (line, column) = buffer.cursor_position();
        let gutter_offset = (buffer.line_count().to_string().len() + 1).max(3) as u16;
        self.draw_cursor(line as u16, column as u16 + gutter_offset - 1);
    }

    fn render_status_line(&mut self, editor: &Editor) {
        let buffer = editor.get_focused_buffer();
        let (width, height) = terminal_size().unwrap();
        let (line, column) = buffer.cursor_position();

        self.draw_status_line(
            buffer.get_path_as_string(),
            buffer.modified,
            line as u16,
            column as u16,
            width,
            height,
        );
    }

    fn render_minibuffer_prompt(&mut self, editor: &Editor, message: &str) {
        let (width, height) = terminal_size().unwrap();

        write!(
            self.stdout,
            "{}{}",
            cursor::Goto(1, height),
            clear::CurrentLine
        )
        .unwrap();
        write!(
            self.stdout,
            "{}{}: {}{}",
            style::Faint,
            message,
            style::Reset,
            editor.minibuffer.text().into_iter().collect::<String>()
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }

    fn clear_minibuffer(&mut self, editor: &Editor) {
        let (width, height) = terminal_size().unwrap();

        write!(
            self.stdout,
            "{}{}",
            cursor::Goto(1, height),
            clear::CurrentLine
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }
}

impl TerminalRenderer {
    pub fn draw_status_line(
        &mut self,
        path: &str,
        modified: bool,
        line: u16,
        column: u16,
        width: u16,
        height: u16,
    ) {
        let mut st = String::new();
        // Drawing the status line
        write!(
            self.stdout,
            "{}",
            cursor::Goto(1 as u16, (height - 1) as u16)
        )
        .unwrap();

        if path == "" {
            st.push_str("[No Name]");
        } else {
            st.push_str(path);
        }
        if modified {
            st.push_str("[+]");
        }

        write!(
            self.stdout,
            "{}{} {} | ({}, {}) ",
            color::Fg(color::Black),
            color::Bg(color::White),
            st,
            line,
            column
        )
        .unwrap();

        let status_info_len = 9 + st.len() + line.to_string().len() + column.to_string().len();
        for i in status_info_len..width as usize {
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

    pub fn draw_line_number(&mut self, line_number: u16, gutter_offset: u16) {
        // Draw line number
        for i in 1..=gutter_offset {
            write!(self.stdout, "{} ", cursor::Goto(i as u16, line_number)).unwrap();
        }

        let number_offset = gutter_offset - (line_number.to_string().len()) as u16;

        write!(
            self.stdout,
            "{}{}{}",
            // color::Fg(color::Rgb(100, 100, 100)),
            style::Faint,
            cursor::Goto(number_offset, line_number),
            line_number
        )
        .unwrap();

        write!(
            self.stdout,
            "{}{}",
            // color::Fg(color::Reset),
            style::Reset,
            color::Bg(color::Reset)
        )
        .unwrap();
    }
}
