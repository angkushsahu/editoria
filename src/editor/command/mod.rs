pub mod edit;
pub mod move_command;
pub mod system;

use crate::editor::{
    command::{edit::Edit, move_command::Move, system::System},
    size::Size,
};
use crossterm::event::Event;

#[derive(Clone, Copy)]
pub enum Command {
    Move(Move),
    Edit(Edit),
    System(System),
}

impl TryFrom<Event> for Command {
    type Error = String;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(key_event) => Edit::try_from(key_event)
                .map(Command::Edit)
                .or_else(|_| Move::try_from(key_event).map(Command::Move))
                .or_else(|_| System::try_from(key_event).map(Command::System))
                .map_err(|_| format!("Event not supported: {key_event:?}")),

            Event::Resize(width_u16, height_u16) => Ok(Self::System(System::Resize(Size {
                height: height_u16 as usize,
                width: width_u16 as usize,
            }))),

            _ => Err(format!("Event not supported: {event:?}")),
        }
    }
}
