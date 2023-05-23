use crate::editor::BufferId;

pub struct View {
    pub buffer_id: BufferId,
    pub surface: Surface,
}

pub struct Surface {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl View {
    pub fn new(buffer_id: BufferId, x: u16, y: u16, width: u16, height: u16) -> Self {
        let surface = Surface {
            x,
            y,
            width,
            height,
        };
        let view = View { buffer_id, surface };
        view
    }
}

