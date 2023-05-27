use std::fs::{read, File};

use std::io::Write;
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

        match &path {
            Some(file_path) => {
                let bytes = read(file_path).expect(format!("Unable to read file: {:?}", file_path).as_str());
                data = bytes
                    .iter()
                    .map(|&byte| byte as char)
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                gap_len = 0;
            }
            None => {
                data = vec![DEFAULT_CHAR; INIT_LEN].into_boxed_slice();
            }
        }

        let mut buffer = Buffer {
            path,
            data,
            gap_start: 0,
            gap_len,
            cursor_offset: 0,
            line_offsets: vec![],
            modified: false,
        };
        buffer.update_line_offsets();
        buffer
    }

    pub fn clear(&mut self) {
        self.data = vec![DEFAULT_CHAR; INIT_LEN].into_boxed_slice();
        self.gap_start = 0;
        self.gap_len = INIT_LEN;
        self.cursor_offset = 0;
        self.line_offsets = vec![];
        self.modified = true;
        self.update_line_offsets();
    }

    pub fn go(&mut self, object: TextObject, direction: Direction) {
        let offset = self.get_object_offset(object, direction);
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
        if new_cursor_offset > (self.data.len() as i32) {
            new_cursor_offset = self.data.len() as i32;
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

        self.assert_dump(
            self.cursor_offset > (self.gap_start + self.gap_len)
                || self.cursor_offset <= self.gap_start,
            "Cursor is in the gap",
        )
        .unwrap();
        self.cursor_offset = new_cursor_offset as usize;

    }

    pub fn delete(&mut self, object: TextObject, direction: Direction) {
        let mut offset = self.get_object_offset(object, direction);

        if offset == 0 {
            return;
        }

        if offset < 0 {
            while offset < 0 {
                if self.cursor_offset == 0 {
                    return;
                };

                self.align_gap();
                self.go(TextObject::Char, Direction::Left);
                self.gap_start -= 1;
                self.gap_len += 1;
                self.update_line_offsets();
                self.modified = true;
                offset += 1;
            }
        } else if offset > 0 {
            while offset > 0 {
                if self.cursor_offset == self.data.len() - self.gap_len {
                    return;
                };

                self.align_gap();
                self.gap_len += 1;
                self.update_line_offsets();
                self.modified = true;
                offset -= 1;
            }
        }
    }

    pub fn insert(&mut self, c: char) {
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
        let expected_len: usize = self.data.len() - self.gap_len;
        let mut text: Box<[char]> = vec![DEFAULT_CHAR; expected_len].into_boxed_slice();

        let mut i: usize = 0;
        let mut j: usize = 0;
        while i < expected_len {
            self.assert_dump(
                i < text.len(),
                "Iterator variable greater than expected text size",
            )
            .unwrap();

            if self.in_gap(j) {
                j += 1;
                continue;
            }

            text[i] = self.data[j];
            i += 1;
            j += 1;
        }
        text
    }

    pub fn text_lines(&self) -> Box<[Box<[char]>]> {
        self.text()
            .split(|c| c == &'\n')
            .collect::<Vec<_>>()
            .iter()
            .map(|ls| ls.to_vec().into_boxed_slice())
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn data_lines(&self) -> Box<[Box<[char]>]> {
        self.data
            .split(|c| c == &'\n')
            .collect::<Vec<_>>()
            .iter()
            .map(|ls| ls.to_vec().into_boxed_slice())
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    // Private interface

    fn in_gap(&self, offset: usize) -> bool {
        self.gap_start <= offset && offset < (self.gap_start + self.gap_len)
    }

    fn align_gap(&mut self) {
        // Aligns the gap with the cursor so that the start of the gap is cursor_pos

        self.assert_dump(
            self.cursor_offset <= self.data.len(),
            "Cursor out of bounds",
        )
        .unwrap();

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

        self.assert_dump(
            self.cursor_offset == self.gap_start,
            "Cursor not aligned with gap\n",
        )
        .unwrap();
    }

    fn grow(&mut self) {
        self.assert_dump(
            self.cursor_offset == self.gap_start,
            "Gap must be aligned before calling grow\n",
        )
        .unwrap();

        self.assert_dump(
            self.gap_len == 0,
            "Gap must be length 0 when grow is called",
        )
        .unwrap();

        // The number of characters to the right of the cursor before grow
        let right_chars_count: usize = self.data.len() - self.gap_start;

        // Reallocation
        let mut new_data: Box<[char]> = vec![DEFAULT_CHAR; self.data.len() + INIT_LEN].into_boxed_slice();
        new_data[..self.data.len()].copy_from_slice(&self.data[..self.data.len()]);
        self.data = new_data;

        // Moving right chars to the end
        self.gap_len = self.data.len() - self.gap_start - right_chars_count;
        self.data.copy_within(
            self.gap_start..(self.gap_start + right_chars_count),
            self.gap_start + self.gap_len,
        )
    }

    fn update_line_offsets(&mut self) {
        self.line_offsets = vec![];

        for i in 0..self.data.len() {
            if self.data[i] == '\n' {
                if i < self.gap_start || i >= self.gap_start + self.gap_len {
                    self.line_offsets.push(i);
                }
            }
        }
    }

    /// Returns the text offset where the next TextObject is in the specific Direction
    ///
    /// # Arguments
    /// * `object` - A TextObject
    /// * `direction` - The direction to find the next object
    ///
    fn get_object_offset(&self, object: TextObject, direction: Direction) -> i32 {
        let mut offset = 0;
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
            },
            TextObject::Word => {
                let word_boundaries: Vec<char> = vec![
                    ' ', '-', ',', '.', ';', ':', '\n', '<', '>', '{', '}', '(', ')', '[', ']',
                ];
                match direction {
                    Direction::Left => {
                        if text.len() == 0 {
                            return 0;
                        }
                        if cursor_offset == text.len() {
                            cursor_offset -= 1;
                        }

                        if cursor_offset as i32 + offset >= 0
                            && !word_boundaries
                                .contains(&text[(cursor_offset as i32 + offset) as usize])
                        {
                            offset -= 1;
                        }

                        while cursor_offset as i32 + offset >= 0
                            && word_boundaries
                                .contains(&text[(cursor_offset as i32 + offset) as usize])
                        {
                            offset -= 1;
                        }

                        while cursor_offset as i32 + offset >= 0
                            && !word_boundaries
                                .contains(&text[(cursor_offset as i32 + offset) as usize])
                        {
                            offset -= 1;
                        }

                        if cursor_offset as i32 + offset >= 0
                            && &text[(cursor_offset as i32 + offset) as usize] == &' '
                        {
                            offset += 1;
                        }
                    }
                    Direction::Right => {
                        // word_boundaries.remove(0);
                        while cursor_offset as i32 + offset < text.len() as i32
                            && word_boundaries
                                .contains(&text[(cursor_offset as i32 + offset) as usize])
                        {
                            offset += 1;
                        }

                        while cursor_offset as i32 + offset < text.len() as i32
                            && !word_boundaries
                                .contains(&text[(cursor_offset as i32 + offset) as usize])
                        {
                            offset += 1;
                        }

                        if cursor_offset as i32 + offset < text.len() as i32
                            && &text[(cursor_offset as i32 + offset) as usize] == &' '
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
                        offset = -(cursor_offset as i32);
                    } else if line <= self.line_offsets.len() {
                        offset = -(self.line_offsets[line - 1] as i32);
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
            },
            TextObject::Paragraph => match direction {
                Direction::Up => {}
                Direction::Down => {}
                Direction::Left => {}
                Direction::Right => {}
            },
        }
        offset
    }

    /// A debugging function which dumps the contents of the Buffer to dump.md before panicing with a message
    ///
    /// # Arguments
    /// * `condition` - the condition that is asserted
    /// * `message` - a message to panic with
    ///
    fn assert_dump(&self, condition: bool, message: &str) -> std::io::Result<()> {
        if !condition {
            let mut file = File::create("dump.md")?;
            file.write_all(format!("data_len: {},\ngap_start: {},\ngap_len: {},\ncursor_offset: {},\nmodified: {},\nline_offsets: {:?},\ndata:\n```rust\n{}\n```",
                self.data.len(),
                self.gap_start,
                self.gap_len,
                self.cursor_offset,
                self.modified,
                self.line_offsets,
                self.data.iter().collect::<String>()).as_bytes())?;

            panic!("{}", message);
        }
        Ok(())
    }
}
