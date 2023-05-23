use std::fs::read;

use std::path::PathBuf;

const INIT_LEN: usize = 20;
const DEFAULT_CHAR: char = '\0';

pub type Position = (usize, usize);
pub type Text = Box<[char]>;

pub enum TextObject {
    Char,
    Word,
    Line,
    Paragraph,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Buffer {
    pub path: Option<PathBuf>,
    pub data: Box<[char]>,
    pub data_len: usize,
    pub gap_start: usize,
    pub gap_len: usize,
    pub cursor_offset: usize,
    pub line_offsets: Vec<usize>,
    pub modified: bool,
}

impl Buffer {
    // Public interface

    pub fn new(path: Option<PathBuf>) -> Self {
        let data: Box<[char]>;
        let mut gap_len = INIT_LEN;
        let mut data_len = INIT_LEN;

        match &path {
            Some(file_path) => {
                if let Ok(bytes) = read(file_path) {
                    let tmp: Vec<char> = bytes.iter().map(|&byte| byte as char).collect();
                    data = tmp.into_boxed_slice();
                    gap_len = 0;
                    data_len = data.len();
                } else {
                    // TODO: Figure out error handling
                    todo!("error handling for invalid files");
                }
            }
            None => {
                data = vec![DEFAULT_CHAR; INIT_LEN].into_boxed_slice();
            }
        }

        let mut buffer = Buffer {
            path,
            data,
            data_len,
            gap_start: 0,
            gap_len,
            cursor_offset: 0,
            line_offsets: vec![],
            modified: false,
        };
        buffer.update_line_offsets();
        buffer
    }

    pub fn get_path_as_string(&self) -> &str {
        match &self.path {
            Some(p) => p.to_str().unwrap(),
            None => "",
        }
    }

    fn get_object_offset(&self, object: TextObject, direction: Direction) -> i32 {
        let mut offset = self.cursor_offset as i32;
        let text = &self.text();
        let mut cursor_offset: usize = self.cursor_offset;
        if cursor_offset > self.gap_start {
            cursor_offset -= self.gap_len;
        }

        match object {
            TextObject::Char => match direction {
                Direction::Left => {
                    offset = -1;
                }
                Direction::Right => {
                    offset = 1;
                }
                _ => {}
            }
            TextObject::Word => {
                let word_boundaries: Vec<char> = vec![' ', '-', '_'];
                match direction {
                    Direction::Left => {
                        offset = -1;
                        while cursor_offset as i32 + offset >= 0
                            && !word_boundaries
                                .contains(&text[(cursor_offset as i32 + offset) as usize])
                        {
                            offset -= 1;
                        }
                    }
                    Direction::Right => {
                        offset = 1;
                        while cursor_offset as i32 + offset < text.len() as i32
                            && !word_boundaries
                                .contains(&text[(cursor_offset as i32 + offset) as usize])
                        {
                            offset += 1;
                        }
                    }
                    _ => {}
                }
            }
            TextObject::Line => match direction {
                Direction::Up => {}
                Direction::Down => {}
                Direction::Left => {
                    let (line, _) = self.cursor_position();

                    if line == 1 {
                        offset = - (cursor_offset as i32);
                    } else if line <= self.line_offsets.len() {
                        offset = - (self.line_offsets[line - 1] as i32);
                    }
                }
                Direction::Right => {
                    let (line, _) = self.cursor_position();

                    if line == self.line_count() {
                        offset = text.len() as i32;
                    } else if line < self.line_offsets.len() {
                        offset = self.line_offsets[line] as i32;
                    }
                }
            }
            TextObject::Paragraph => match direction {
                Direction::Up => {}
                Direction::Down => {}
                Direction::Left => {}
                Direction::Right => {}
            }
        }
        offset
    }

    pub fn go(&mut self, object: TextObject, direction: Direction) {
        let target = self.get_object_offset(object, direction);
        self.move_cursor(target);
    }

    pub fn delete(&mut self, object: TextObject, direction: Direction) {
        let mut target = self.get_object_offset(object, direction);
        
        if target == 0 {
            return;
        }

        while target != 0 {
            if target < 0 {
                self.delete_char_backward();
                target += 1;
            } else if target > 0 {
                self.delete_char_forward();
                target -= 1;
            }
        }
    }

    pub fn select(&mut self, object: TextObject, direction: Direction) {
        todo!("selection");
        let target = self.get_object_offset(object, direction);
    }

    pub fn insert_char(&mut self, c: char) {
        self.align_gap();

        // Buffer out of space
        if self.gap_len == 0 {
            self.grow();
        }

        self.data[self.gap_start] = c;
        self.gap_start += 1;
        self.cursor_offset += 1;
        self.gap_len -= 1;
        self.update_line_offsets();
        self.modified = true
    }

    pub fn move_cursor(&mut self, offset: i32) {
        let mut new_cursor_offset: i32 = self.cursor_offset as i32 + offset;

        // Only the cases where the cursor offset changes sides relative to the gap start should be handled

        // From before the gap to after the gap
        if self.cursor_offset <= self.gap_start && (self.gap_start as i32) < new_cursor_offset {
            new_cursor_offset += self.gap_len as i32;
        }

        // From after the gap to before the gap
        if new_cursor_offset <= (self.gap_start as i32) && self.gap_start < self.cursor_offset {
            new_cursor_offset -= self.gap_len as i32;
        }

        // Boundary cases
        if new_cursor_offset < 0 {
            new_cursor_offset = 0;
        }
        if new_cursor_offset > (self.data_len as i32) {
            new_cursor_offset = self.data_len as i32;
        }

        // Cursor goes into the gap from the left
        // HACK: Not sure if `&& self.gap_len != 1` works
        if new_cursor_offset == (self.gap_start + 1) as i32 && self.gap_len != 1 {
            new_cursor_offset += self.gap_len as i32;
        }
        // Cursor goes into the gap from the right
        if new_cursor_offset == (self.gap_start + self.gap_len) as i32 {
            new_cursor_offset = self.gap_start as i32;
        }

        assert!(
            self.cursor_offset > (self.gap_start + self.gap_len)
                || self.cursor_offset <= self.gap_start,
            "Cursor is in the gap"
        );
        self.cursor_offset = new_cursor_offset as usize;
    }

    pub fn go_to_line(&self, dest_line: usize) {
        todo!("go_to_line");
    }

    pub fn go_to_column(&self, column: usize) {
        todo!("go_to_column");
    }

    pub fn move_to_eol(&mut self) {
        todo!("move_to_eol");
    }

    pub fn move_to_bol(&mut self) {
        todo!("move_to_bol");
    }

    pub fn kill_to_eol(&mut self) {
        todo!("kill_to_eol");
    }

    pub fn kill_to_bol(&mut self) {
        todo!("kill_to_bol");
    }

    pub fn delete_char_forward(&mut self) {
        if self.cursor_offset == self.data_len - self.gap_len {
            return;
        };

        self.align_gap();
        self.gap_len += 1;
        self.update_line_offsets();
        self.modified = true
    }

    pub fn delete_char_backward(&mut self) {
        if self.cursor_offset == 0 {
            return;
        };

        self.align_gap();
        self.move_cursor(-1);
        self.gap_start -= 1;
        self.gap_len += 1;
        self.update_line_offsets();
        self.modified = true
    }

    pub fn line_count(&self) -> usize {
        self.line_offsets.len() + 1
    }

    pub fn cursor_position(&self) -> Position {
        let mut line: usize = 0;

        while line < self.line_count() - 1 && self.cursor_offset > self.line_offsets[line] {
            line += 1;
        }

        let mut column: usize = self.cursor_offset;

        if line > 0 {
            column -= self.line_offsets[line - 1] + 1;
        }

        // Cursor is in front of gap and gap is on the same line
        if line == 0 {
            if column > self.gap_start {
                column -= self.gap_len;
            }
        } else {
            let line_offset = self.line_offsets[line - 1];
            if line_offset < self.gap_start && self.gap_start < self.cursor_offset {
                column -= self.gap_len;
            }
        }

        (line + 1, column + 1)
    }

    pub fn text(&self) -> Text {
        let expected_len: usize = self.data_len - self.gap_len;
        let mut text: Box<[char]> = vec![DEFAULT_CHAR; expected_len].into_boxed_slice();

        let mut i: usize = 0;
        let mut j: usize = 0;
        while i < expected_len {
            assert!(
                i < text.len(),
                "Iterator variable greater than expected text size"
            );

            if self.gap_start <= j && j < self.gap_start + self.gap_len {
                j += 1;
                continue;
            }

            text[i] = self.data[j];
            i += 1;
            j += 1;
        }
        text
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
        self.line_offsets = vec![];

        for i in 0..self.data_len {
            if self.data[i] == '\n' {
                if i < self.gap_start || i >= self.gap_start + self.gap_len {
                    self.line_offsets.push(i);
                }
            }
        }
    }
}
