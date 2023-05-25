path: Some("src/buffer.rs"),
data_len: 15332,
gap_start: 514,
gap_len: 18,
cursor_offset: 531,
modified: true,
line_offsets: [31, 32, 52, 76, 77, 105, 138, 139, 175, 204, 205, 227, 237, 247, 257, 272, 274, 275, 296, 304, 314, 324, 335, 337, 338, 355, 375, 406, 433, 458, 484, 508, 553, 587, 611, 613, 614, 628, 652, 653, 701, 732, 768, 805, 806, 828, 861, 914, 1005, 1056, 1089, 1132, 1157, 1212, 1275, 1293, 1307, 1329, 1401, 1415, 1425, 1426, 1460, 1478, 1496, 1518, 1544, 1565, 1595, 1629, 1658, 1669, 1707, 1722, 1728, 1729, 1776, 1803, 1847, 1871, 1881, 1887, 1888, 1975, 1983, 2003, 2037, 2099, 2107, 2190, 2218, 2251, 2310, 2354, 2397, 2407, 2408, 2431, 2481, 2518, 2551, 2569, 2607, 2639, 2657, 2681, 2696, 2730, 2785, 2881, 2900, 2934, 2975, 3020, 3058, 3084, 3141, 3189, 3215, 3216, 3278, 3326, 3417, 3443, 3484, 3510, 3511, 3576, 3623, 3714, 3740, 3781, 3807, 3808, 3873, 3921, 4012, 4038, 4079, 4105, 4106, 4168, 4255, 4281, 4322, 4348, 4370, 4412, 4466, 4546, 4593, 4684, 4710, 4751, 4777, 4778, 4858, 4906, 4997, 5023, 5064, 5090, 5091, 5168, 5255, 5281, 5322, 5348, 5370, 5398, 5416, 5430, 5480, 5516, 5554, 5591, 5651, 5652, 5687, 5745, 5809, 5881, 5903, 5921, 5959, 6019, 6020, 6071, 6123, 6186, 6251, 6273, 6291, 6306, 6361, 6397, 6435, 6473, 6512, 6527, 6537, 6552, 6558, 6559, 6628, 6692, 6726, 6732, 6733, 6806, 6874, 6875, 6900, 6920, 6930, 6931, 6955, 6986, 7031, 7060, 7074, 7105, 7136, 7180, 7209, 7223, 7233, 7239, 7240, 7313, 7341, 7405, 7411, 7412, 7457, 7483, 7484, 7515, 7546, 7571, 7581, 7582, 7621, 7650, 7683, 7710, 7746, 7775, 7781, 7782, 7831, 7908, 7909, 8017, 8018, 8066, 8163, 8217, 8227, 8228, 8276, 8373, 8427, 8437, 8438, 8464, 8499, 8534, 8544, 8600, 8654, 8664, 8665, 8715, 8773, 8856, 8910, 8920, 8971, 9044, 9099, 9109, 9110, 9136, 9201, 9258, 9293, 9313, 9370, 9376, 9377, 9426, 9455, 9461, 9462, 9510, 9541, 9547, 9548, 9584, 9614, 9620, 9621, 9665, 9729, 9749, 9760, 9761, 9787, 9814, 9850, 9879, 9885, 9886, 9931, 9968, 9988, 9999, 10000, 10026, 10056, 10085, 10112, 10148, 10177, 10183, 10184, 10224, 10260, 10266, 10267, 10315, 10348, 10349, 10442, 10465, 10475, 10476, 10528, 10529, 10551, 10606, 10616, 10617, 10682, 10705, 10746, 10786, 10800, 10817, 10876, 10961, 11001, 11015, 11025, 11026, 11057, 11063, 11064, 11097, 11161, 11250, 11251, 11281, 11311, 11344, 11374, 11406, 11474, 11498, 11499, 11573, 11597, 11623, 11637, 11638, 11674, 11694, 11714, 11724, 11737, 11743, 11744, 11769, 11770, 11800, 11885, 11886, 11982, 11983, 12058, 12059, 12082, 12102, 12112, 12113, 12135, 12173, 12211, 12212, 12247, 12328, 12379, 12394, 12395, 12444, 12473, 12511, 12579, 12580, 12644, 12645, 12680, 12756, 12788, 12803, 12804, 12864, 12913, 12923, 12924, 12950, 13000, 13044, 13064, 13070, 13071, 13096, 13122, 13172, 13228, 13248, 13249, 13347, 13348, 13423, 13494, 13495, 13519, 13563, 13598, 13692, 13760, 13790, 13791, 13832, 13907, 13938, 14004, 14047, 14057, 14063, 14064, 14104, 14140, 14141, 14177, 14215, 14293, 14340, 14358, 14372, 14382, 14388, 14389, 14499, 14507, 14527, 14582, 14628, 14636, 14719, 14743, 14796, 14972, 14999, 15030, 15062, 15092, 15128, 15159, 15194, 15262, 15263, 15298, 15308, 15323, 15329, 15331],
data:
```rust
use std::fs::{read, somethign};

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
    pub data_len: usize,
    pub gap_start: usize,
    pub gap_len: usize,
    pub usize,
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

    pub fn go(&mut self, object: TextObject, direction: Direction) {
        let target = self.get_object_offset(object, direction);
        self.move_cursor(target);
    }

    pub fn delete(&mut self, object: TextObject, direction: Direction) {
        let mut offset = self.get_object_offset(object, direction);

        if offset == 0 {
            return;
        }

        if offset < 0 {
            while offset < 0 {
                self.delete_char_backward();
                offset += 1;
            }
        } else if offset > 0 {
            while offset > 0 {
                self.delete_char_forward();
                offset -= 1;
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

        self.assert_dump(
            self.cursor_offset > (self.gap_start + self.gap_len)
                || self.cursor_offset <= self.gap_start,
            "Cursor is in the gap"
        ).unwrap();
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
            self.assert_dump(
                i < text.len(),
                "Iterator variable greater than expected text size"
            ).unwrap();

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

        self.assert_dump(self.cursor_offset <= self.data_len, "Cursor out of bounds").unwrap();

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
            "Cursor not aligned with gap\n"
        ).unwrap();
    }

    fn grow(&mut self) {
        self.assert_dump(
            self.cursor_offset == self.gap_start,
            "Gap must be aligned before calling grow\n"
        ).unwrap();

        self.assert_dump(self.gap_len == 0, "Gap must be length 0 when grow is called").unwrap();

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

    /// A debugging function which dumps the contents of the Buffer to dump.md before panicing with a message
    ///
    /// # Arguments
    /// * `condition` - the condition that is asserted
    /// * `message` - a message to panic with
    ///
    fn assert_dump(&self, condition: bool, message: &str) -> std::io::Result<()> {
        if !condition {
            let mut file = File::create("dump.md")?;
            file.write_all(format!("path: {:?},\ndata_len: {},\ngap_start: {},\ngap_len: {},\ncursor_offset: {},\nmodified: {},\nline_offsets: {:?},\ndata:\n```rust\n{}\n```",
                self.path,
                self.data_len,
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

```