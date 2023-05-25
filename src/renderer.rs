use termion::{
    clear, color, cursor,
    raw::{IntoRawMode, RawTerminal},
};

use std::io::{stdout, Stdout, Write};
use termion::screen::{AlternateScreen, IntoAlternateScreen};

use crate::editor::Editor;

pub trait Renderer {
    fn new() -> Self;
    fn render(&mut self, editor: &Editor);
    fn render_cursor(&mut self, editor: &Editor);
    fn render_status_line(&mut self, editor: &Editor);
    fn render_minibuffer_prompt(&mut self, editor: &Editor, message: String, response: String);
}

pub struct TerminalRenderer {
    // stdout: AlternateScreen<RawTerminal<Stdout>>,
    stdout: RawTerminal<Stdout>,
    pub debug: bool,
}

impl Renderer for TerminalRenderer {
    fn new() -> Self {
        TerminalRenderer {
            stdout: stdout()
                .into_raw_mode()
                .unwrap(),
                // .into_alternate_screen()
                // .unwrap(),
            // stdout: stdout().into_raw_mode().unwrap(),
            debug: false,
        }
    }

    fn render(&mut self, editor: &Editor) {
        // let view = editor.views.get(&editor.focused).unwrap(); // TODO: Change unwrap
        // let buffer = editor.buffers.get(&view.buffer_id).unwrap();
        let view = editor.get_focused_view();
        let buffer = editor.get_focused_buffer();

        let surface = &view.surface;
        let text = &buffer.text();

        let (width, height) = ((surface.x + surface.width), (surface.y + surface.height));
        let (line, column) = buffer.cursor_position();
        let gutter_offset = (buffer.line_count().to_string().len() + 1).max(3) as u16;

        let mut line_number: u16 = 1;
        let mut line_offset = 0;

        write!(self.stdout, "{}", cursor::Goto(1, 1)).unwrap();
        write!(self.stdout, "{}", cursor::Hide).unwrap();

        // TODO: Show name of file on top
        // write!(self.stdout, "{}", buffer.get_path_as_string()).unwrap();
        // line_number += 1;


        if text.len() == 0 {
            write!(self.stdout, "{}", clear::AfterCursor).unwrap();
        } else {
            self.draw_line_number(line_number, gutter_offset);
        }

        // Main loop for rendering text
        for i in 0..text.len() {
            // TODO: Make sure column is less than width
            if line_number > height - 1 {
                break;
            }

            if text[i] == '\n' {
                // Clear everything that was not overwritten in the current line render call
                write!(
                    self.stdout,
                    "{}{}",
                    cursor::Goto((i - line_offset + 1) as u16 + gutter_offset, line_number),
                    clear::UntilNewline
                )
                .unwrap();

                line_number += 1;
                line_offset = i + 1;

                self.draw_line_number(line_number, gutter_offset);
                continue;
            }

            // Display the actual character
            write!(
                self.stdout,
                "{}{}",
                cursor::Goto((i - line_offset + 1) as u16 + gutter_offset, line_number),
                text[i]
            )
            .unwrap();

            // use std::{thread, time};
            // let ten_millis = time::Duration::from_millis(1000);
            // thread::sleep(ten_millis);
            // self.stdout.flush().unwrap();
        }
        write!(
            self.stdout,
            "{}{}",
            cursor::Goto(
                (text.len() - line_offset + 1) as u16 + gutter_offset,
                line_number
            ),
            clear::UntilNewline
        )
        .unwrap();

        line_number += 1;
        while line_number <= height - 1 {
            write!(
                self.stdout,
                "{}{}",
                cursor::Goto(1, line_number),
                clear::CurrentLine
            ).unwrap();
            line_number += 1;
        }
        // Clear everything that was not overwritten after the last char
        // write!(
        //     self.stdout,
        //     "{}{}",
        //     cursor::Goto(
        //         (text.len() - line_offset + 1) as u16 + gutter_offset,
        //         line_number
        //     ),
        //     clear::AfterCursor
        // )
        // .unwrap();

        self.draw_status_line(
            buffer.get_path_as_string(),
            buffer.modified,
            line as u16,
            column as u16,
            width,
            height,
        );

        if self.debug {
            self.draw_debug_info(editor, line_number);
        }

        self.draw_cursor(line as u16, column as u16 + gutter_offset);
        self.stdout.flush().unwrap();
    }

    fn render_cursor(&mut self, editor: &Editor) {
        let buffer = editor.get_focused_buffer();

        let (line, column) = buffer.cursor_position();
        let gutter_offset = (buffer.line_count().to_string().len() + 1).max(3) as u16;
        self.draw_cursor(line as u16, column as u16 + gutter_offset);
    }

    fn render_status_line(&mut self, editor: &Editor) {
        let view = editor.get_focused_view(); // TODO: Change unwrap
        let buffer = editor.get_focused_buffer();

        let surface = &view.surface;
        let (width, height) = ((surface.x + surface.width), (surface.y + surface.height));

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

    fn render_minibuffer_prompt(&mut self, editor: &Editor, message: String, response: String) {
        let view = editor.get_focused_view(); // TODO: Change unwrap
        let buffer = editor.get_focused_buffer();

        let surface = &view.surface;
        let (_, height) = ((surface.x + surface.width), (surface.y + surface.height));

        // Drawing the status line
        write!(self.stdout, "{}", cursor::Goto(1 as u16, (height - 1) as u16)).unwrap();
        write!(self.stdout, "{}", clear::CurrentLine).unwrap();

        write!(
            self.stdout,
            "{}{} {}: {}",
            color::Fg(color::Black),
            color::Bg(color::White),
            message,
            response
        ).unwrap();

        write!(
            self.stdout,
            "{}{}",
            color::Bg(color::Reset),
            color::Fg(color::Reset)
        ).unwrap();
    }
}

impl TerminalRenderer {
    fn draw_debug_info(&mut self, editor: &Editor, line_number: u16) {
        // let view = editor.views.get(&editor.focused).unwrap(); // TODO: Change unwrap
        // let buffer = editor.buffers.get(&view.buffer_id).unwrap();

        let view = editor.get_focused_view();
        let buffer = editor.get_focused_buffer();

        let surface = &view.surface;
        let text = &buffer.data;

        let (width, height) = ((surface.x + surface.width), (surface.y + surface.height));
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
            "{}WIDTH: {}, HEIGHT: {})",
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
            buffer.data_len
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
        write!(self.stdout, "{}", cursor::Goto(1 as u16, (height - 1) as u16)).unwrap();

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
        ).unwrap();


        let status_info_len = 9 + st.len() + line.to_string().len() + column.to_string().len();
        for i in status_info_len..width as usize {
            write!(self.stdout, "{}", cursor::Goto(i as u16, (height - 1) as u16)).unwrap();
            write!(
                self.stdout,
                " ",
            ).unwrap();
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
            color::Fg(color::Rgb(100, 100, 100)),
            cursor::Goto(number_offset, line_number),
            line_number
        )
        .unwrap();

        write!(
            self.stdout,
            "{}{}",
            color::Fg(color::Reset),
            color::Bg(color::Reset)
        )
        .unwrap();
    }
}
