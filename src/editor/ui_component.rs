use std::io::Error;

use crate::editor::size::Size;

pub trait UiComponent {
    fn set_needs_redraw(&mut self, value: bool);

    fn needs_redraw(&self) -> bool;

    fn set_size(&mut self, size: Size);

    fn resize(&mut self, size: Size) {
        self.set_size(size);
        self.set_needs_redraw(true);
    }

    fn draw(&mut self, origin_row: usize) -> Result<(), Error>;

    fn render(&mut self, origin_row: usize) {
        if self.needs_redraw() {
            match self.draw(origin_row) {
                Ok(()) => self.set_needs_redraw(false),
                #[allow(unused_variables)]
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not render component: {err:?}");
                    }
                }
            }
        }
    }
}
