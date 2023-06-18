use std::{fs::read, path::PathBuf};
use log::*;

const INIT_LEN: usize = 10;
const DEFAULT_CHAR: char = '\0';

pub type Position = (usize, usize);
pub type Selection = (Position, Position);

#[derive(Clone, Copy)]
pub enum TextObject {
    Char,
    Word,
    Line,
}

#[derive(Clone, Copy)]
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
    pub modified: bool,
    pub mark: Option<Position>,
    pub clipboard: Option<Box<[char]>>,
}

impl Buffer {
    pub fn new(path: Option<PathBuf>) -> Self {
        let mut data: Box<[char]> = vec![DEFAULT_CHAR; INIT_LEN].into_boxed_slice();
        let mut gap_len = INIT_LEN;

        if let Some(file_path) = &path {
            if file_path.exists() {
                gap_len = 0;
                data = read(&file_path)
                    .expect(format!("Unable to read file: {:?}", file_path).as_str())
                    .iter()
                    .map(|&byte| byte as char)
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
            }
        }

        Buffer {
            path,
            data,
            gap_start: 0,
            gap_len,
            cursor_offset: 0,
            modified: false,
            mark: None,
            clipboard: None,
        }
    }

    pub fn clear(&mut self) {
        self.data = vec![DEFAULT_CHAR; INIT_LEN].into_boxed_slice();
        self.gap_start = 0;
        self.gap_len = INIT_LEN;
        self.cursor_offset = 0;
        self.modified = true;
    }

    // ---------- Editing ----------

    pub fn go(&mut self, object: TextObject, direction: Direction) {
        let offset = self.get_object_offset(object, direction);
        let mut new_cursor_offset = self.cursor_offset as i32 + offset;
        let gap_end = (self.gap_start + self.gap_len) as i32;

        // From before the gap to after the gap
        if self.cursor_offset <= self.gap_start && (self.gap_start as i32) < new_cursor_offset {
            new_cursor_offset += self.gap_len as i32;
        }

        // From after the gap to before the gap
        if new_cursor_offset <= gap_end && gap_end < self.cursor_offset as i32 {
            new_cursor_offset -= self.gap_len as i32;
        }

        new_cursor_offset = new_cursor_offset.max(0).min(self.data.len() as i32);

        if new_cursor_offset == gap_end {
            new_cursor_offset = self.gap_start as i32;
        }

        self.cursor_offset = new_cursor_offset as usize;
        self.align_gap();

        assert!(!self.in_gap(self.cursor_offset), "Cursor in gap");
    }

    pub fn delete(&mut self, object: TextObject, direction: Direction) {
        let offset = self
            .get_object_offset(object.clone(), direction.clone())
            .max(-1 * self.cursor_offset as i32)
            .min((self.data.len() - self.gap_len - self.cursor_offset) as i32);

        self.align_gap();

        if offset < 0 {
            self.gap_start = (self.gap_start as i32 + offset) as usize;
            self.cursor_offset = self.gap_start;
        }

        self.gap_len += offset.abs() as usize;
        self.modified = true;
    }

    pub fn insert(&mut self, c: char) {
        // Buffer out of space
        if self.gap_len == 0 {
            self.grow();
        }

        self.align_gap();
        self.data[self.gap_start] = c;
        self.gap_start += 1;
        self.cursor_offset += 1;
        self.gap_len -= 1;
        self.modified = true;
    }

    pub fn go_to_start(&mut self) {
        self.cursor_offset = 0;
        self.align_gap();
    }

    pub fn go_to_end(&mut self) {
        self.cursor_offset = self.data.len();
        self.align_gap();
    }

    // ---------- Selection ----------

    pub fn toggle_selection(&mut self) {
        match self.mark {
            None => self.mark = Some(self.cursor_position()),
            Some(_) => self.mark = None,
        }
    }

    pub fn get_selection(&self) -> Option<Selection> {
        if let Some(mark) = self.mark {
            let cursor_position = self.cursor_position();
            let (mark_line, mark_col) = mark;
            let (cursor_line, cursor_col) = cursor_position;

            if mark_line == cursor_line {
                if mark_col >= cursor_col {
                    return Some((cursor_position, mark));

                } else if mark_col < cursor_col {
                    return Some((mark, cursor_position));
                }
            } else if mark_line > cursor_line {
                return Some((cursor_position, mark));

            } else if mark_line < cursor_line {
                return Some((mark, cursor_position));
            }
        }
        None
    }

    pub fn copy_to_clipboard(&mut self) {
        if let Some(mark) = self.mark {
            // TODO: This doesn't need to call text(), optimise later!
            let text = self.text();
            let mark_offset = text
                .iter()
                .enumerate()
                .filter(|(i, c)| c == &&'\n' || i == &0)
                .nth(mark.0 - 1)
                .unwrap()
                .0
                + mark.1
                - 1;
            let (a, b) = (mark_offset.min(self.cursor_offset), mark_offset.max(self.cursor_offset));

            let mut new_clipboard = text[a..b].iter().map(|c| c.to_owned()).collect::<Vec<_>>();
            if !new_clipboard.is_empty() && new_clipboard[0] == '\n' {
                new_clipboard.remove(0);
            }

            self.clipboard = Some(new_clipboard.into_boxed_slice());
            self.toggle_selection();
        }
    }

    pub fn paste_from_clipboard(&mut self) {
        let clipboard = self.clipboard.clone();
        if let Some(text) = clipboard {
            for c in text.iter() {
                self.insert(*c);
            }
        }
    }

    pub fn delete_selection(&mut self) {
        if let Some(mark) = self.mark {
            let mark_offset = self
                .text()
                .iter()
                .enumerate()
                .filter(|(i, c)| c == &&'\n' || i == &0)
                .nth(mark.0 - 1)
                .unwrap()
                .0
                + mark.1
                - 1 + (mark.0 != 1) as usize;

            let mut diff = self.cursor_offset as i32 - mark_offset as i32;
            while diff > 0 {
                self.delete(TextObject::Char, Direction::Left);
                diff -= 1;
            }
            while diff < 0 {
                self.delete(TextObject::Char, Direction::Right);
                diff += 1;
            }
            self.toggle_selection();
        }
    }

    // ---------- Accessing content ----------

    pub fn line_count(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .filter(|(i, c)| c == &&'\n' && !self.in_gap(*i + 1))
            .count()
    }

    pub fn text(&self) -> Box<[char]> {
        self.data
            .into_iter()
            .enumerate()
            .filter(|(i, _)| !self.in_gap(*i + 1))
            .map(|(_, x)| x.to_owned())
            .collect::<Vec<char>>()
            .into_boxed_slice()
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

    pub fn cursor_position(&self) -> Position {
        assert!(self.is_aligned(), "Gap must be aligned");

        let text = self.text();

        let line = 1 + text
            .iter()
            .take(self.cursor_offset)
            .filter(|c| c == &&'\n')
            .count();

        let column = self.cursor_offset as i32
            - text
                .iter()
                .take(self.cursor_offset)
                .enumerate()
                .filter(|(_, c)| c == &&'\n')
                .map(|(i, _)| i)
                .fold(None, |_, x| Some(x as i32))
                .unwrap_or(-1);
        (line, column as usize)
    }

    // ---------- Accessing debug content ----------

    pub fn text_raw(&self) -> Box<[char]> {
        self.data
            .into_iter()
            .enumerate()
            .map(|(i, c)| {
                if self.in_gap(i + 1) {
                    '_'
                } else {
                    c.to_owned()
                }
            })
            .collect::<Vec<char>>()
            .into_boxed_slice()
    }

    pub fn text_lines_raw(&self) -> Box<[Box<[char]>]> {
        self.text_raw()
            .split(|c| c == &'\n')
            .collect::<Vec<_>>()
            .iter()
            .map(|ls| ls.to_vec().into_boxed_slice())
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    // ---------- Helper functions ----------

    fn is_aligned(&self) -> bool {
        self.gap_start == self.cursor_offset
    }

    pub fn in_gap(&self, offset: usize) -> bool {
        //   012345678
        //  [hell____o]
        //       ^   ^
        //       |   |
        // gap_start |
        //           |
        //   (gap_start + gap_len)
        //
        // [5, 6, 7, 8] are in gap

        self.gap_start < offset && offset <= (self.gap_start + self.gap_len)
    }

    fn align_gap(&mut self) {
        assert!(
            self.cursor_offset <= self.data.len(),
            "Cursor out of bounds"
        );

        let diff: i32 = self.cursor_offset as i32 - self.gap_start as i32;

        if diff < 0 {
            // before: [--|--__________-----]
            // after : [--|_________--------]

            self.data.copy_within(
                self.cursor_offset..(self.cursor_offset as i32 - diff) as usize,
                self.cursor_offset + self.gap_len,
            );
            self.gap_start = self.cursor_offset;
        } else if diff > 0 {
            // before: [-----__________---|-]
            // after : [--------|_________--]

            let gap_end: usize = self.gap_start + self.gap_len;

            self.data.copy_within(
                gap_end..(gap_end + (diff - self.gap_len as i32) as usize),
                self.gap_start,
            );
            self.gap_start += diff as usize - self.gap_len;
            self.cursor_offset = self.gap_start;
        }

        assert!(self.is_aligned(), "align_gap did not align properly.");
    }

    fn grow(&mut self) {
        assert_eq!(
            self.cursor_offset, self.gap_start,
            "Gap must be aligned before calling grow\n"
        );

        assert_eq!(self.gap_len, 0, "Gap must be length 0 when grow is called",);

        // The number of characters to the right of the cursor before grow
        let right_chars_count: usize = self.data.len() - self.gap_start;

        // Reallocation
        let mut new_data: Box<[char]> =
            vec![DEFAULT_CHAR; self.data.len() + INIT_LEN].into_boxed_slice();
        new_data[..self.data.len()].copy_from_slice(&self.data[..self.data.len()]);
        self.data = new_data;

        // Moving right chars to the end
        self.gap_len = self.data.len() - self.gap_start - right_chars_count;
        self.data.copy_within(
            self.gap_start..(self.gap_start + right_chars_count),
            self.gap_start + self.gap_len,
        )
    }

    fn get_object_offset(&self, object: TextObject, direction: Direction) -> i32 {
        let mut offset: i32 = 0;

        match object {
            TextObject::Char => match direction {
                Direction::Left => offset = -1,
                Direction::Right => offset = 1,
                _ => {}
            },
            TextObject::Word => {
                let word_boundaries = [
                    ' ', '!', '\"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.',
                    '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{',
                    '|', '}', '~', '\n',
                ];
                let text = self.text();
                match direction {
                    Direction::Left => {
                        let first_char = text
                            .iter()
                            .take(self.cursor_offset)
                            .rev()
                            .enumerate()
                            .find(|(_, c)| !word_boundaries.contains(c))
                            .map(|(i, _)| i)
                            .unwrap_or(0);

                        offset = text
                            .iter()
                            .take(self.cursor_offset - first_char)
                            .rev()
                            .enumerate()
                            .find(|(_, c)| word_boundaries.contains(c))
                            .map(|(i, _)| i)
                            .unwrap_or(self.cursor_offset) as i32
                            + first_char as i32;

                        offset *= -1;
                    }
                    Direction::Right => {
                        let first_char = text
                            .iter()
                            .skip(self.cursor_offset)
                            .enumerate()
                            .find(|(_, c)| !word_boundaries.contains(c))
                            .map(|(i, _)| i)
                            .unwrap_or(text.len() - self.cursor_offset);

                        offset = text
                            .iter()
                            .skip(first_char + self.cursor_offset)
                            .enumerate()
                            .find(|(_, c)| word_boundaries.contains(c))
                            .map(|(i, _)| i)
                            .unwrap_or(text.len() - self.cursor_offset)
                            as i32
                            + first_char as i32;
                    }
                    _ => {}
                }
            }
            TextObject::Line => {
                let text_lines = self.text_lines();
                let (line, column) = self.cursor_position();

                match direction {
                    Direction::Up => {
                        if line == 1 {
                            return offset;
                        }

                        offset = -1
                            * text_lines
                                .iter()
                                .nth(line - 2)
                                .map(|previous_line| {
                                    (column + previous_line.len()
                                        - previous_line.len().min(column - 1))
                                        as i32
                                })
                                .unwrap_or(0);
                    }
                    Direction::Down => {
                        offset = text_lines
                            .iter()
                            .nth(line)
                            .map(|next_line| {
                                (text_lines
                                    .iter()
                                    .nth(line - 1)
                                    .expect("Could not get current line")
                                    .len()
                                    + next_line.len().min(column).max(1)
                                    - (column - 1)) as i32
                            })
                            .unwrap_or(0);
                    }
                    Direction::Left => {
                        offset = -1 * column as i32 + 1;
                    }
                    Direction::Right => {
                        offset = (text_lines
                            .iter()
                            .nth(line - 1)
                            .unwrap_or(&Box::new(vec![].into_boxed_slice()))
                            .len()
                            + 1
                            - column) as i32
                    }
                }
            }
        }
        offset
    }
}
