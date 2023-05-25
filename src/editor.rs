use crate::buffer::Buffer;
use crate::view::View;
use std::collections::HashMap;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub type ViewId = usize;
pub type BufferId = usize;

pub struct Editor {
    buffers: HashMap<BufferId, Buffer>,
    next_buffer_id: BufferId,
    views: HashMap<ViewId, View>,
    next_view_id: ViewId,
    focused: ViewId,
}

pub struct MiniBuffer {
    message: String,
    buffer: Buffer,
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

    pub fn get_focused_view(&self) -> &View {
        self.views.get(&self.focused).unwrap() // TODO: Change unwrap
    }

    pub fn get_focused_buffer(&self) -> &Buffer {
        let view = self.views.get(&self.focused).unwrap(); // TODO: Change unwrap
        self.buffers.get(&view.buffer_id).unwrap()
    }

    pub fn get_focused_buffer_mut(&mut self) -> &mut Buffer {
        let view = self.views.get(&self.focused).unwrap(); // TODO: Change unwrap
        self.buffers.get_mut(&view.buffer_id).unwrap()
    }

    pub fn open_file(&mut self, path: Option<PathBuf>) {
        self.buffers.insert(self.next_buffer_id, Buffer::new(path));

        // TODO: This should not stay as the max height and width
        let (width, height) = termion::terminal_size().unwrap();
        self.views.insert(
            self.next_view_id,
            View::new(self.next_buffer_id, 0, 0, width, height),
        );

        self.focused = self.next_view_id;

        self.next_buffer_id += 1;
        self.next_view_id += 1;
    }

    pub fn save_buffer(&mut self) {
        let buffer = self.get_focused_buffer_mut();

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
}
