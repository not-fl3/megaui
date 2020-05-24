#[derive(Clone, Copy, Debug)]
pub enum KeyCode {
    Up,
    Down,
    Right,
    Left,
    Backspace,
    Delete,
    Enter,
    Home,
    End,
}

pub trait InputHandler {
    fn mouse_down(&mut self, position: (f32, f32));
    fn mouse_up(&mut self, _: (f32, f32));
    fn mouse_wheel(&mut self, x: f32, y: f32);
    fn mouse_move(&mut self, position: (f32, f32));
    fn char_event(&mut self, character: char);
    fn key_down(&mut self, key_down: KeyCode, shift: bool, ctrl: bool);
}
