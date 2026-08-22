use crate::{editor::annotated_string::annotated_type::AnnotationType, types::ByteIndex};

#[derive(Copy, Clone, Debug)]
pub struct Annotation {
    pub annotation_type: AnnotationType,
    pub start: ByteIndex,
    pub end: ByteIndex,
}
