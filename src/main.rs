mod editor;
mod terminal;

use editor::Editor;

fn main() {
    // using unwrap makes sense because we need to crash if something really messed up happens and
    // the editor cannot enter raw mode
    Editor::new().unwrap().run();
}
