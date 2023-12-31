use std::ops::Range;

pub type Completions = Vec<CompletionItem>;

// TODO: make this more similar to
// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#completionItem.

#[derive(Debug)]
pub struct CompletionItem {
    /// The text to display in the details window as a vector of strings.
    pub details: Option<Vec<String>>,

    /// The formatted completion item as shown inside the completion menu.
    pub format: String,

    /// TODO: docs
    pub matched_prefix: u16,

    /// The number of bytes before the current cursor position that are
    /// matched by the completion item.
    pub matched_bytes: Vec<Range<usize>>,

    /// The name of the source this completion comes from.
    pub source: &'static str,

    /// The text that will be inserted into the buffer if the completion is
    /// selected.
    pub text: String,
}
