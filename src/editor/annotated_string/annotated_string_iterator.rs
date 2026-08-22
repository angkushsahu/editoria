use crate::{
    editor::annotated_string::{annotated_string_part::AnnotatedStringPart, AnnotatedString},
    types::ByteIndex,
};
use std::cmp::min;

pub struct AnnotatedStringIterator<'a> {
    pub annotated_string: &'a AnnotatedString,
    pub current_index: ByteIndex,
}

impl<'a> Iterator for AnnotatedStringIterator<'a> {
    type Item = AnnotatedStringPart<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.annotated_string.string.len() {
            return None;
        }

        let last_annotation = self
            .annotated_string
            .annotations
            .iter()
            .rfind(|annotation| {
                annotation.start <= self.current_index && annotation.end > self.current_index
            });

        if let Some(annotation) = last_annotation {
            let end_idx = min(annotation.end, self.annotated_string.string.len());

            let start_idx = self.current_index;
            self.current_index = end_idx;
            return Some(AnnotatedStringPart {
                string: &self.annotated_string.string[start_idx..end_idx],
                annotation_type: Some(annotation.annotation_type),
            });
        }

        let mut end_idx = self.annotated_string.string.len();
        for annotation in &self.annotated_string.annotations {
            if annotation.start > self.current_index && annotation.start < end_idx {
                end_idx = annotation.start;
            }
        }

        let start_idx = self.current_index;
        self.current_index = end_idx;

        Some(AnnotatedStringPart {
            string: &self.annotated_string.string[start_idx..end_idx],
            annotation_type: None,
        })
    }
}
