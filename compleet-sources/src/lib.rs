pub mod lipsum;
pub mod lsp;

mod completion_item;
mod completion_source;
mod cursor;
mod valid_source;

pub mod prelude {
    pub use crate::completion_item::{CompletionItem, Completions};
    pub use crate::completion_source::{CompletionSource, Sources};
    pub use crate::cursor::Cursor;
    pub use crate::valid_source::ValidSource;
}
