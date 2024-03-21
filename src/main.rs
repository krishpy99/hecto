#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
// NOTE: As a learning exercise, have Clippy warn about everything.
// Learn why it's warning, and then decide if it's a good idea to keep the warning on or not.
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else,
    clippy::min_ident_chars,
    clippy::question_mark_used,
    clippy::pub_use,
    clippy::std_instead_of_core,
    clippy::as_conversions,
    clippy::partial_pub_fields,
    clippy::exhaustive_structs,
    clippy::exhaustive_enums,
    clippy::pattern_type_mismatch,
    clippy::panic
)]
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
