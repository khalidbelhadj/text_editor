// TODO: Use the correct access modifiers for struct fields
// TODO: Chage default char
use crate::renderer::Renderer;
use std::path::PathBuf;

const INIT_LEN: usize = 10;
const CURSOR_STR: char = '|';
const GAP_STR: char = '_';
const RENDER_NEW_LINES: bool = true;
const DEFAULT_CHAR: char = '_';

pub type Position = (usize, usize);

pub type Text = Box<[char]>;

#[derive(Debug)]
pub struct Buffer {
    path: Option<PathBuf>,
    modified: bool, // TODO: Figure out if this is even needed
    pub data: Box<[char]>, // TODO: Remove pub
    data_len: usize,
    pub gap_start: usize, // TODO: Remove pub
    pub gap_len: usize, // TODO: Remove pub
    pub cursor_offset: usize, // TODO: Remove pub
    pub line_offsets: Vec<usize>,
}

impl Buffer {
    // Public interface

    pub fn new(path: Option<PathBuf>) -> Self {
        let data: Box<[char]>;

        match path {
            Some(path) => {
                todo!("New gap buffer from file path not implemented");
            }
            None => {
                data = vec![DEFAULT_CHAR; INIT_LEN].into_boxed_slice();
            }
        }

        Buffer {
            path,
            modified: false,
            data,
            data_len: INIT_LEN,
            gap_start: 0,
            gap_len: INIT_LEN,
            cursor_offset: 0,
            line_offsets: vec![],
        }
    }

    pub fn save(&mut self) {
        self.modified = false;
        todo!("save not implemented");
    }

    pub fn insert_char(&mut self, c: char) {
        self.align_gap();

        // Buffer out of space
        if self.gap_len == 0 {
            self.grow();
        }

        self.data[self.gap_start] = c;
        self.gap_start += 1;
        self.gap_len -= 1;
        self.cursor_offset += 1;
        self.modified = true;
        self.update_line_offsets();
    }

    pub fn move_cursor(&mut self, offset: i32) {
        let mut new_cursor_pos: i32 = self.cursor_offset as i32 + offset;

        if new_cursor_pos < 0 {
            new_cursor_pos = 0;
        } else if new_cursor_pos > self.data_len as i32 {
            new_cursor_pos = self.data_len as i32;
        } else if self.gap_len != 0 {
            if new_cursor_pos == (self.gap_start + self.gap_len) as i32 {
                assert_ne!(offset, 0, "Cursor got in that ugly spot");
                new_cursor_pos = self.gap_start as i32;
            } else if new_cursor_pos == (self.gap_start + 1) as i32 {
                assert_ne!(offset, 0, "Cursor got into the gap");
                if (self.gap_start + self.gap_len) != self.data_len {
                    new_cursor_pos = (self.gap_start + self.gap_len + 1) as i32;
                } else {
                    new_cursor_pos = self.cursor_offset as i32;
                }
            }
        }

        self.cursor_offset = new_cursor_pos as usize;
    }

    pub fn move_to_eol(&mut self) {
        self.align_gap();

        let (line, column) = self.cursor_position();
        let mut offset = self.data_len - self.cursor_offset - self.gap_len;

        if self.line_offsets.len() != 0 {
            // offset = (self.line_offsets[line - 1] - self.cursor_offset);
            // offset = 0;
        }

        self.move_cursor(offset as i32);
    }

    pub fn move_to_bol(&mut self) {
        todo!("move_to_bol");
    }

    pub fn delete_char_forward(&mut self) {
        if self.cursor_offset == self.data_len - self.gap_len {return};

        self.align_gap();
        self.gap_len += 1;
    }

    pub fn delete_char_backward(&mut self) {
        if self.cursor_offset == 0 {return};

        self.align_gap();
        self.move_cursor(-1);
        self.gap_start -= 1;
        self.gap_len += 1;
    }

    pub fn line_count(&self) -> usize {
        self.line_offsets.len() + 1
    }

    pub fn cursor_position(&self) -> Position {
        let mut line_number: usize = 0;

        while line_number < self.line_count() - 1
            && self.cursor_offset > self.line_offsets[line_number]
        {
            line_number += 1;
        }

        // let column_number: usize = self.cursor_offset - self.line_offsets[(line_number - 1).max(0)].min;
        let mut column_number: usize = self.cursor_offset;
        if line_number > 0 {
            column_number -= self.line_offsets[line_number - 1];
        }

        if column_number > self.gap_start {
            column_number -= self.gap_start;
        }
        (line_number + 1, column_number)
    }

    // Private interface

    fn align_gap(&mut self) {
        // Aligns the gap with the cursor so that the start of the gap is cursor_pos

        assert!(self.cursor_offset <= self.data_len, "Cursor out of bounds");

        let diff: i32 = self.cursor_offset as i32 - self.gap_start as i32;

        if diff == 0 {
            return;
        }

        if diff < 0 {
            // [--|--__________-----]
            // [-----|_________-----]

            self.data.copy_within(
                self.cursor_offset..(self.cursor_offset as i32 - diff) as usize,
                self.cursor_offset + self.gap_len,
            );

            self.gap_start = self.cursor_offset;
        } else if diff > 0 {
            // [-----__________---|-]
            // [-----_________|-----] This case should never happen

            let gap_end: usize = self.gap_start + self.gap_len;

            self.data.copy_within(
                gap_end..(gap_end + (diff - self.gap_len as i32) as usize),
                self.gap_start,
            );

            self.gap_start += diff as usize - self.gap_len;
            self.cursor_offset = self.gap_start;
        }

        assert_eq!(
            self.cursor_offset, self.gap_start,
            "Cursor not aligned with gap\n"
        );
    }

    fn grow(&mut self) {
        assert_eq!(
            self.cursor_offset, self.gap_start,
            "Gap must be aligned before calling grow\n"
        );

        assert_eq!(self.gap_len, 0, "Gap must be length 0 when grow is called");

        // The number of characters to the right of the cursor before grow
        let right_chars_count: usize = self.data_len - self.gap_start;

        // Reallocation
        let old_len: usize = self.data_len;
        self.data_len += INIT_LEN;
        let mut new_data: Box<[char]> = vec![DEFAULT_CHAR; self.data_len].into_boxed_slice();
        new_data[..old_len].copy_from_slice(&self.data[..old_len]);
        self.data = new_data;

        // Moving right chars to the end
        self.gap_len = self.data_len - self.gap_start - right_chars_count;
        self.data.copy_within(
            self.gap_start..(self.gap_start + right_chars_count),
            self.gap_start + self.gap_len,
        )
    }

    fn update_line_offsets(&mut self) {
        let prev_line_count: usize = self.line_offsets.len();
        let mut line_number: usize = 0;

        for i in 0..self.data_len {
            if self.data[i] == '\n' {
                if line_number < prev_line_count {
                    self.line_offsets[line_number] = i;
                } else {
                    self.line_offsets.push(i);
                }
                line_number += 1;
            }
        }

        if line_number < prev_line_count {
            for _ in 0..prev_line_count - line_number - 1 {
                self.line_offsets.pop();
            }
        }
    }

    // TODO: Only for debugging, delete when not needed
    pub fn print_debug(&self) {
        for i in 0..self.data_len {
            if i == self.cursor_offset {
                print!("{}", CURSOR_STR);
                continue;
            }
            if self.gap_start <= i && i < self.gap_start + self.gap_len {
                print!("{}", GAP_STR);
                continue;
            }
            if self.data[i] == '\n' && !RENDER_NEW_LINES {
                print!("\\n");
                continue;
            }
            print!("{}", self.data[i]);
        }
        println!(
            "\ndata_len: {}, cursor_pos: {}, gap_start: {}, gap_len: {}\n",
            self.data_len, self.cursor_offset, self.gap_start, self.gap_len
        );
    }
}

pub struct View {
    pub buffer: Buffer,
    pub surface: Surface,
    pub line_wrapping: bool,
}

pub struct Surface {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl View {
    pub fn new(buffer: Buffer, x: usize, y: usize, width: usize, height: usize) -> Self {
        let surface = Surface { x, y, width, height };
        let mut view = View {
            buffer,
            surface,
            line_wrapping: true,
        };
        view
    }

    pub fn text(&self) -> Text {
        let expected_len: usize = self.buffer.data_len - self.buffer.gap_len;
        let mut text: Box<[char]> = vec![DEFAULT_CHAR; expected_len].into_boxed_slice();

        let mut i: usize = 0;
        let mut j: usize = 0;
        while i < expected_len {
            assert!(
                i < text.len(),
                "Iterator variable greater than expected text size"
            );

            if self.buffer.gap_start <= j && j < self.buffer.gap_start + self.buffer.gap_len {
                j += 1;
                continue;
            }

            text[i] = self.buffer.data[j];
            i += 1;
            j += 1;
        }
        text
    }

    pub fn cursor_offset(&self) -> usize {
        let mut x: usize = self.buffer.cursor_offset;
        if x > self.buffer.gap_start {
            x -= self.buffer.gap_len;
        }
        x
    }
}

pub struct Editor {
    buffers: Vec<Buffer>,
    views: Vec<View>,
}
