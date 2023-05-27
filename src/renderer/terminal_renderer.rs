use crate::{renderer::Renderer, controller::EditorState, buffer::Buffer};
use termion::{
    clear, color, cursor,
    raw::{IntoRawMode, RawTerminal},
    style, terminal_size,
};

use std::{
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
            // stdout: stdout()
            //     .into_raw_mode()
            //     .unwrap()
            //     .into_alternate_screen()
            //     .unwrap()
            stdout: stdout().into_raw_mode().unwrap()
        }
    }

    fn render(&mut self, editor: &Editor) {
        let buffer = editor.get_focused_buffer();
        let (width, height) = terminal_size().expect("Could not get terminal size");
        let lines = buffer.text_lines();
        let lines_trimmed = lines.iter().take((height - 2)as usize).collect::<Vec<_>>();
        let gutter_offset = (buffer.line_count().to_string().len() + 1).max(3) as u16;

        // if lines_trimmed[0].len() == 0 {
        //     write!(self.stdout, "{}", clear::All).unwrap();
        //     return;
        // }


        write!(self.stdout, "{}", cursor::Hide).unwrap();

        for i in 0..lines_trimmed.len() {
            let line_number = (i + 1) as u16;
            let line = lines_trimmed[i]
                .iter()
                .take(width as usize)
                .collect::<String>();


            // Display the line
            write!(self.stdout, "{}{}", cursor::Goto(gutter_offset + 1, line_number), line).unwrap();

            // Clear everything that wasn't overwritten in the current line
            write!(self.stdout,"{}{}", cursor::Goto(width.min(line.len() as u16 + gutter_offset + 1) as u16, line_number), clear::UntilNewline).unwrap();

            // Drawing the line number

            // Clearing the gutter
            for i in 1..=gutter_offset {
                write!(self.stdout, "{} ", cursor::Goto(i as u16, line_number)).unwrap();
            }

            let number_offset = gutter_offset - (line_number.to_string().len()) as u16;

            write!(self.stdout, "{}{}{}", style::Faint, cursor::Goto(number_offset, line_number), line_number).unwrap();
            write!(self.stdout, "{}{}", style::Reset, color::Bg(color::Reset)).unwrap();
            // write!(self.stdout, "{}", gutter_offset).unwrap();
        }

        // Clearing all lines until the status line
        for line in (lines_trimmed.len() + 1) as u16..=(height - 2) {
            write!(self.stdout, "{}{}", cursor::Goto(1, line), clear::CurrentLine).unwrap();
        }

        self.render_status_line(&editor);
        self.render_cursor(&editor);
        self.stdout.flush().unwrap();
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
        write!(self.stdout, "{}", cursor::Goto(column as u16 + gutter_offset, line as u16)).unwrap();
        write!(self.stdout, "{}", cursor::Show).unwrap();
        write!(self.stdout, "{}", cursor::BlinkingBlock).unwrap();
        self.stdout.flush().unwrap();
    }

    fn render_status_line(&mut self, editor: &Editor) {
        let buffer = editor.get_focused_buffer();
        let (width, height) = terminal_size().expect("Could not get terminal size");
        let (line, column) = buffer.cursor_position();
        let mut file_name = String::new();

        write!(self.stdout, "{}", cursor::Hide).unwrap();
        write!(self.stdout, "{}", cursor::Goto(1 as u16, (height - 1) as u16)).unwrap();

        match &buffer.path {
            Some(p) => file_name.push_str(p.to_str().unwrap()),
            None => file_name.push_str("[No Name]"),
        }

        if buffer.modified { file_name.push_str("[+]"); }

        write!(self.stdout, "{}{} {} | ({}, {}) ", color::Fg(color::Black), color::Bg(color::White), file_name, line, column).unwrap();

        let status_info_len = 9 + file_name.len() + line.to_string().len() + column.to_string().len();
        for i in status_info_len..width as usize {
            write!(self.stdout, "{} ", cursor::Goto(i as u16, (height - 1) as u16)).unwrap();
        }

        write!(self.stdout, "{}{}", color::Bg(color::Reset), color::Fg(color::Reset)).unwrap();
        write!(self.stdout, "{}", cursor::Show).unwrap();
        self.stdout.flush().unwrap();
    }

    fn render_minibuffer_prompt(&mut self, editor: &Editor, message: &str) {
        let (width, height) = terminal_size().unwrap();

        write!(self.stdout, "{}{}", cursor::Goto(1, height), clear::CurrentLine).unwrap();
        write!(self.stdout, "{}{}: {}{}", style::Faint, message, style::Reset, editor.minibuffer.text().into_iter().collect::<String>()).unwrap();
        self.stdout.flush().unwrap();
    }

    fn clear_minibuffer(&mut self, editor: &Editor) {
        let (width, height) = terminal_size().unwrap();

        write!(self.stdout, "{}{}", cursor::Goto(1, height), clear::CurrentLine).unwrap();
        self.stdout.flush().unwrap();
    }
}
