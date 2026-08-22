use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Clone, Copy)]
pub enum Edit {
    Insert(char),
    InsertNewLine,
    Delete,
    DeleteBackward,
}

impl TryFrom<KeyEvent> for Edit {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        match (event.code, event.modifiers) {
            (KeyCode::Char(character), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                Ok(Self::Insert(character))
            }
            (KeyCode::Tab, KeyModifiers::NONE) => Ok(Self::Insert('\t')),
            (KeyCode::Enter, KeyModifiers::NONE) => Ok(Self::InsertNewLine),
            (KeyCode::Backspace, KeyModifiers::NONE) => Ok(Self::DeleteBackward),
            (KeyCode::Delete, KeyModifiers::NONE) => Ok(Self::Delete),
            _ => Err(format!(
                "Unsupported key code {:?} with modifiers {:?}",
                event.code, event.modifiers
            )),
        }
    }
}
