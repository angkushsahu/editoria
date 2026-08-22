use crate::types::{GraphemeIndex, LineIndex};

#[derive(Copy, Clone, Default)]
pub struct Location {
    pub grapheme_index: GraphemeIndex,
    pub line_index: LineIndex,
}
