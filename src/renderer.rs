use crate::buffer::{Text, View};

pub trait Renderer {
    fn render_view(&self, buffer: &View);
}

pub struct NCursesRenderer;

impl Renderer for NCursesRenderer {
    fn render_view(&self, view: &View) {
        let text: Text = view.text();

        for i in 0..text.len() {
            if i == view.cursor_offset() {
                print!("|");
                continue;
            }
            print!("{}", text[i]);
        }
        println!();
    }
}
