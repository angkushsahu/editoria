use crate::{editor::line::GraphemeWidth, types::ByteIndex};

#[derive(Clone, Debug)]
pub struct TextFragment {
    pub grapheme: String,
    pub rendered_width: GraphemeWidth,
    pub replacement: Option<char>,
    pub start: ByteIndex,
}
