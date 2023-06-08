pub mod terminal_renderer;
pub mod debug_terminal_renderer;

use crate::editor::Editor;

pub trait Renderer {
    fn new() -> Self where Self: Sized;
    fn render(&mut self, editor: &Editor);

    fn render_cursor(&mut self, editor: &Editor);
    fn render_status_line(&mut self, editor: &Editor);
    fn render_minibuffer_prompt(&mut self, editor: &Editor, message: &str);
    fn clear_minibuffer(&mut self, editor: &Editor);
}
