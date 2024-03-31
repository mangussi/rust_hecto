#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else,
    clippy::absolute_paths,
    clippy::single_call_fn,
    clippy::let_underscore_untyped
)]

mod document;
mod editor;
mod row;
mod terminal;
pub use document::Document;
pub use editor::Editor;
pub use editor::Position;
pub use editor::SearchDirection;
pub use row::Row;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
