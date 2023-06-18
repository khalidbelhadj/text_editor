use crate::{buffer::Buffer, controller::EditorState, renderer::Renderer};
use log::info;
use termion::{
    clear, color, cursor,
    raw::{IntoRawMode, RawTerminal},
    style, terminal_size,
};

use std::{
    io::{stdout, Stdout, Write},
    ops::Div,
};
use termion::screen::{AlternateScreen, IntoAlternateScreen};

use crate::editor::Editor;

pub const STATUS_BAR_HEIGHT: u16 = 2;

pub struct TerminalRenderer {
    stdout: AlternateScreen<RawTerminal<Stdout>>, // stdout: RawTerminal<Stdout>,
    window_start: u16,
}

impl Renderer for TerminalRenderer {
    fn new() -> Self {
        TerminalRenderer {
            stdout: stdout()
                .into_raw_mode()
                .unwrap()
                .into_alternate_screen()
                .unwrap(), // stdout: stdout().into_raw_mode().unwrap(),
            window_start: 0,
        }
    }

    fn render_all(&mut self, editor: &Editor) {
        self.render_editor(&editor);
        self.render_status_line(&editor);
        self.render_cursor(&editor);

        self.stdout.flush().unwrap();
    }

    fn render_editor(&mut self, editor: &Editor) {
        self.update_window(&editor);
        let buffer = editor.get_focused_buffer();
        let (width, height) = terminal_size().expect("Could not get terminal size");
        let lines = buffer.text_lines();
        let lines_trimmed = lines // TODO: This doesn't need to be a copy, use slice instead
            .iter()
            .enumerate()
            .map(|(i, line)| (i + 1, line))
            .skip(self.window_start as usize)
            .take((height - STATUS_BAR_HEIGHT) as usize)
            .collect::<Vec<_>>();
        let gutter_offset = self.gutter_offset(&editor) as u16;

        let selection = buffer.get_selection();

        write!(self.stdout, "{}", cursor::Hide).unwrap();

        for i in 0..lines_trimmed.len() {
            let line_position = (i + 1) as u16;
            let line_number = lines_trimmed[i].0 as u16;
            let line = lines_trimmed[i]
                .1
                .iter()
                .take(width as usize)
                .collect::<String>();

            // Draw line content and selection if needed
            for (i, c) in line.chars().enumerate() {
                if let Some(((a1, a2), (b1, b2))) = selection {
                    let in_selection = ((a1 + 1)..=(b1 - 1)).contains(&(line_number as usize))
                        || ((line_number as usize == a1 && i + 1 >= a2 && a1 != b1)
                            || (line_number as usize == b1 && i + 1 < b2 && a1 != b1))
                        || (line_number as usize == a1 && a1 == b1 && a2 <= i + 1 && i + 1 < b2);

                    if in_selection {
                        write!(self.stdout, "{}", color::Bg(color::LightBlue)).unwrap();
                    } else {
                        write!(self.stdout, "{}", color::Bg(color::Reset)).unwrap();
                    }
                }
                write!(
                    self.stdout,
                    "{}{c}",
                    cursor::Goto(gutter_offset + 1 + i as u16, line_position)
                )
                .unwrap();
            }

            write!(self.stdout, "{}", color::Bg(color::Reset)).unwrap();

            // Clearing everything that was not overwritten
            write!(
                self.stdout,
                "{}{}",
                cursor::Goto(gutter_offset + 1 + line.len() as u16, line_position),
                clear::UntilNewline
            )
            .unwrap();

            // Clearing the gutter
            for i in 1..=gutter_offset {
                write!(
                    self.stdout,
                    "{}{} ",
                    color::Bg(color::Reset),
                    cursor::Goto(i as u16, line_number)
                )
                .unwrap();
            }

            // Drawing the line number
            let number_offset = gutter_offset - (line_number.to_string().len()) as u16;
            write!(
                self.stdout,
                "{}{}{}",
                style::Faint,
                cursor::Goto(number_offset, line_position),
                line_number
            )
            .unwrap();
            write!(self.stdout, "{}", style::Reset).unwrap();
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

        self.stdout.flush().unwrap();
    }

    fn render_line(&mut self, editor: &Editor) {
        let buffer = editor.get_focused_buffer();
        let lines = buffer.text_lines();
        let (mut line_number, _) = buffer.cursor_position();
        let line = lines.iter().collect::<Vec<_>>()[line_number - 1];
        line_number -= 1;

        write!(
            self.stdout,
            "{}{}{}{}",
            cursor::Goto(
                self.gutter_offset(&editor) as u16 + 1,
                line_number as u16 - self.window_start + 1
            ),
            cursor::Hide,
            clear::UntilNewline,
            line.iter().collect::<String>()
        )
        .unwrap();

        self.render_cursor(&editor);

        self.stdout.flush().unwrap();
    }

    fn render_cursor(&mut self, editor: &Editor) {
        self.update_window(&editor);
        let buffer = editor.get_focused_buffer();

        let (line_number, column_number) = buffer.cursor_position();
        let gutter_offset = (buffer.line_count().to_string().len() + 1).max(3) as u16;

        write!(
            self.stdout,
            "{}{}{}",
            cursor::Goto(
                column_number as u16 + gutter_offset,
                line_number as u16 - self.window_start
            ),
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
            cursor::Goto(
                (width as usize - status_info_right.len()) as u16,
                (height - 1) as u16
            ),
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
            "{}{}",
            color::Bg(color::Reset),
            color::Fg(color::Reset),
        )
        .unwrap();

        self.stdout.flush().unwrap();
    }

    fn render_minibuffer_prompt(&mut self, editor: &Editor, message: &str) {
        let (_, height) = terminal_size().unwrap();

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

    fn clear_minibuffer(&mut self, _: &Editor) {
        let (_, height) = terminal_size().unwrap();

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
            let to_center_it = line as i32 - window_height.div(2) as i32;
            self.window_start = to_center_it.max(0) as u16;
        } else if line as u16 > window_end {
            let to_center_it = line as i32 + window_height.div(2) as i32;
            self.window_start = to_center_it.max(0) as u16;
        }

        if old_window_start != self.window_start {
            info!(
                "window start updated from {} to {}",
                old_window_start, self.window_start
            );
            self.render_editor(editor);
        }
    }

    fn gutter_offset(&self, editor: &Editor) -> usize {
        editor
            .get_focused_buffer()
            .line_count()
            .to_string()
            .len()
            .max(2)
            + 1
    }
}
