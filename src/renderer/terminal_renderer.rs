use crate::{buffer::Buffer, controller::EditorState, renderer::Renderer, editor};
use log::info;
use termion::{
    clear, color, cursor,
    raw::{IntoRawMode, RawTerminal},
    style, terminal_size,
};

use std::{
    fmt::format,
    io::{stdout, Stdout, Write}, ops::Div,
};
use termion::screen::{AlternateScreen, IntoAlternateScreen};

use crate::editor::Editor;

pub const STATUS_BAR_HEIGHT: u16 = 2;

pub struct TerminalRenderer {
    stdout: AlternateScreen<RawTerminal<Stdout>>, // stdout: RawTerminal<Stdout>,
    window_start: u16
}

impl Renderer for TerminalRenderer {
    fn new() -> Self {
        TerminalRenderer {
            stdout: stdout()
                .into_raw_mode()
                .unwrap()
                .into_alternate_screen()
                .unwrap(), // stdout: stdout().into_raw_mode().unwrap(),
            window_start: 0
        }
    }

    fn render(&mut self, editor: &Editor) {
        let buffer = editor.get_focused_buffer();
        let (width, height) = terminal_size().expect("Could not get terminal size");
        self.update_window(&editor);
        
        let lines = buffer.text_lines();
        let lines_trimmed = lines.iter().skip(self.window_start as usize).take((height - 2) as usize).collect::<Vec<_>>();
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
            let number_offset = gutter_offset - ((line_number + self.window_start).to_string().len()) as u16;
            write!(
                self.stdout,
                "{}{}{}",
                style::Faint,
                cursor::Goto(number_offset, line_number),
                line_number + self.window_start
            )
            .unwrap();
            write!(self.stdout, "{}{}", style::Reset, color::Bg(color::Reset)).unwrap();
        }

        // Clearing all lines until the status line
        for line in (lines_trimmed.len() + 1) as u16..=(height - STATUS_BAR_HEIGHT) {
            write!(
                self.stdout,
                "{}{}",
                cursor::Goto(1, line),
                clear::CurrentLine
            )
            .unwrap();
        }

        self.render_status_line(&editor);
        self.render_cursor(&editor);
        self.stdout.flush().unwrap();
    }

    fn render_cursor(&mut self, editor: &Editor) {
        self.update_window(&editor);
        let buffer: &Buffer;
        let state = editor.state;

        match state {
            EditorState::Editing => buffer = editor.get_focused_buffer(),
            EditorState::PromptResponse => buffer = &editor.minibuffer,
        }

        let (line, column) = buffer.cursor_position();
        let gutter_offset = (buffer.line_count().to_string().len() + 1).max(3) as u16;

        write!(
            self.stdout,
            "{}{}{}",
            cursor::Goto(column as u16 + gutter_offset, line as u16 - self.window_start),
            cursor::Show,
            cursor::BlinkingBlock
        )
        .unwrap();

        self.stdout.flush().unwrap();
    }

    fn render_status_line(&mut self, editor: &Editor) {
        self.update_window(&editor);
        let buffer = editor.get_focused_buffer();
        let (width, height) = terminal_size().expect("Could not get terminal size");
        let (line, column) = buffer.cursor_position();
        let mut file_name = String::new();

        // Generating file name as string
        match &buffer.path {
            Some(p) => file_name.push_str(p.to_str().unwrap()),
            None => file_name.push_str("[No Name]"),
        }

        if buffer.modified {
            file_name.push_str("[+]");
        }

        // Drawing the status info
        let status_info_left = format!(" {} ", file_name);
        let status_info_right = format!(" {}:{} ", line, column);

        write!(self.stdout, "{}", cursor::Hide).unwrap();
        
        write!(
            self.stdout,
            "{}{}{}{}",
            cursor::Goto(1 as u16, (height - 1) as u16),
            color::Fg(color::Black),
            color::Bg(color::White),
            status_info_left
        )
        .unwrap();
        
        write!(
            self.stdout,
            "{}{}{}{}",
            cursor::Goto((width as usize - status_info_right.len()) as u16, (height - 1) as u16),
            color::Fg(color::Black),
            color::Bg(color::White),
            status_info_right
        )
        .unwrap();

        // Drawing the gap and resetting
        for i in status_info_left.len()..(width as usize) - status_info_right.len() {
            write!(
                self.stdout,
                "{} ",
                cursor::Goto(i as u16, (height - 1) as u16)
            )
            .unwrap();
        }

        write!(
            self.stdout,
            "{}{}{}",
            color::Bg(color::Reset),
            color::Fg(color::Reset),
            cursor::Show
        )
        .unwrap();
       
        self.stdout.flush().unwrap();
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
    fn update_window(&mut self, editor: &Editor) {
        let old_window_start = self.window_start;
        let buffer = editor.get_focused_buffer();
        let (_, height) = terminal_size().expect("Could not get terminal size");
        let (line, _) = buffer.cursor_position();
        let window_height = height - STATUS_BAR_HEIGHT;
        let window_end = self.window_start + window_height;

        if line as u16 <= self.window_start {
            self.window_start -= window_height.div(2).min(self.window_start);
        } else if line as u16 > window_end {
            self.window_start += window_height.div(2).min(buffer.line_count() as u16 - window_end);
        }

        if old_window_start != self.window_start {
            info!("window start updated from {} to {}", old_window_start, self.window_start);
            self.render(editor);
        }
    }
}
