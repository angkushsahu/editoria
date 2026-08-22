use crate::editor::annotated_string::annotated_type::AnnotationType;

#[derive(Debug)]
pub struct AnnotatedStringPart<'a> {
    pub string: &'a str,
    pub annotation_type: Option<AnnotationType>,
}
