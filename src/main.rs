#![warn(clippy::all, clippy::pedantic)]
mod document;
mod editor;
mod row;
mod terminal;
pub use document::Document;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

use editor::Editor;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
