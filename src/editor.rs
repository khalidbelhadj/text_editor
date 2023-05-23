use std::collections::HashMap;
use crate::buffer::{Buffer, TextObject, Direction};
use crate::view::View;
use crate::controller::{EditorCommand, ControlCommand, RedrawItem};

use termion::event::Key;

use std::io::Write;
use std::path::PathBuf;
use std::fs::File;

pub type ViewId = usize;
pub type BufferId = usize;

pub struct Editor {
    pub buffers: HashMap<BufferId, Buffer>,
    next_buffer_id: BufferId,
    pub views: HashMap<ViewId, View>,
    next_view_id: ViewId,
    pub focused: ViewId,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            buffers: HashMap::new(),
            next_buffer_id: 0,
            views: HashMap::new(),
            next_view_id: 0,
            focused: 0,
        }
    }

    pub fn open_file(&mut self, path: Option<PathBuf>) {
        // TODO: Handle option from insert
        self.buffers.insert(self.next_buffer_id, Buffer::new(path));
        let (width, height) = termion::terminal_size().unwrap();
        self.views.insert(self.next_view_id, View::new(self.next_buffer_id, 0, 0, width, height));

        self.focused = self.next_view_id;

        self.next_buffer_id += 1;
        self.next_view_id += 1;
    }

    pub fn save_buffer(&mut self) {
        let view = self.views.get(&self.focused).unwrap(); // TODO: Change unwrap
        let buffer = self.buffers.get_mut(&view.buffer_id).unwrap();

        match &buffer.path {
            Some(file_path) => {
                // TODO: Remove unwraps and actually handle the errors
                let mut f = File::create(file_path).unwrap();
                let data = buffer.text().iter().collect::<String>();
                f.write_all(data.as_bytes()).expect("Unable to write data");
                buffer.modified = false
            }
            None => {
                todo!("handling saving with no path");
            }
        }
    }

    // TODO: Not sure if I should be using a Result for this
    // TODO: At least use proper generics inside the Result
    pub fn handle_key(&mut self, key: Key) -> ControlCommand {
        let view = self.views.get(&self.focused).unwrap(); // TODO: Change unwrap
        let buffer = self.buffers.get_mut(&view.buffer_id).unwrap();

        match key {
            Key::Char(c) => {
                buffer.insert_char(c);
                return ControlCommand::Redraw(RedrawItem::FocusedView);
            }
            Key::Left => {
                buffer.go(TextObject::Char, Direction::Left);
                return ControlCommand::Redraw(RedrawItem::Cursor);
            }
            Key::Right => {
                buffer.go(TextObject::Char, Direction::Right);
                return ControlCommand::Redraw(RedrawItem::Cursor);
            }
            Key::Backspace => {
                buffer.delete(TextObject::Char, Direction::Left);
                return ControlCommand::Redraw(RedrawItem::FocusedView);
            }
            Key::Ctrl(c) => {
                match c {
                    'c' => {
                        return ControlCommand::Editor(EditorCommand::Quit)
                    }
                    'd' => {
                        buffer.delete(TextObject::Char, Direction::Right);
                        return ControlCommand::Redraw(RedrawItem::FocusedView);
                    }
                    'b' => {
                        buffer.go(TextObject::Char, Direction::Left);
                        return ControlCommand::Redraw(RedrawItem::Cursor);
                    }
                    'f' => {
                        buffer.go(TextObject::Char, Direction::Right);
                        return ControlCommand::Redraw(RedrawItem::Cursor);
                    }
                    'e' => {
                        buffer.go(TextObject::Line, Direction::Right);
                        return ControlCommand::Redraw(RedrawItem::Cursor);
                    }
                    'a' => {
                        buffer.go(TextObject::Line, Direction::Left);
                        return ControlCommand::Redraw(RedrawItem::Cursor);
                    }
                    'w' => {
                        self.save_buffer();
                        return ControlCommand::Redraw(RedrawItem::StatusLine);
                    }
                    'z' => {
                        buffer.go(TextObject::Word, Direction::Left);
                        return ControlCommand::Redraw(RedrawItem::Cursor);
                    }
                    'x' => {
                        buffer.go(TextObject::Word, Direction::Right);
                        return ControlCommand::Redraw(RedrawItem::Cursor);
                    }
                    'y' => {
                        self.buffers.remove(&view.buffer_id);
                        self.buffers.insert(view.buffer_id, Buffer::new(None));
                    }
                    _ => {
                        todo!("Ctrl modifier not implemented")
                    }
                }
            }
            _ => {
                // todo!("key not handled key: {:?}", key);
            }
        }
        return ControlCommand::Noop;
    }
}
