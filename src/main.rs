#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
pub use editor::Position;
pub use terminal::Terminal;

use editor::Editor;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
