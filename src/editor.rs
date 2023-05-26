use crate::buffer::Buffer;
use crate::controller::EditorState;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

pub type BufferId = usize;

pub struct Editor {
    buffers: HashMap<BufferId, Buffer>,
    next_buffer_id: BufferId,
    pub focused: BufferId,
    pub minibuffer: Buffer,
    pub state: EditorState,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            buffers: HashMap::new(),
            next_buffer_id: 1,
            focused: 0,
            minibuffer: Buffer::new(None),
            state: EditorState::Editing,
        }
    }

    pub fn get_focused_buffer(&self) -> &Buffer {
        return self.buffers.get(&self.focused).unwrap();
    }

    pub fn get_focused_buffer_mut(&mut self) -> &mut Buffer {
        return self.buffers.get_mut(&self.focused).unwrap();
    }

    pub fn get_minibuffer(&mut self) -> &mut Buffer {
        &mut self.minibuffer
    }

    pub fn get_state(&mut self) -> EditorState {
        self.state
    }

    pub fn open_file(&mut self, path: Option<PathBuf>) {
        self.buffers.insert(self.next_buffer_id, Buffer::new(path));
        self.focused = self.next_buffer_id;
        self.next_buffer_id += 1;
    }

    pub fn save_buffer(&mut self, new_path: Option<String>) {
        let buffer = self.get_focused_buffer_mut();

        match new_path {
            Some(path) => {
                buffer.path = Some(PathBuf::from_str(path.as_str()).unwrap());
            }
            None => {}
        }

        match &buffer.path {
            Some(file_path) => {
                // TODO: Remove unwraps and actually handle the errors
                let mut f = File::create(file_path).expect(format!("Something messed up with the path {:?}", file_path).as_str());
                let data = &buffer.text().iter().collect::<String>();
                f.write_all(data.as_bytes()).expect("Unable to write data");
                buffer.modified = false
            }
            None => {
                todo!("handling saving with no path");
            }
        }
    }
}
