use crate::types::Vector2;

#[derive(Clone, Debug)]
pub enum InputCharacter {
    Char(char),
    ControlCode(crate::input_handler::KeyCode),
}

#[derive(Default, Clone)]
pub struct Input {
    pub mouse_position: Vector2,
    pub is_mouse_down: bool,
    pub click_down: bool,
    pub click_up: bool,
    pub mouse_wheel: Vector2,
    pub input_buffer: Vec<InputCharacter>,
}

impl Input {
    pub fn reset(&mut self) {
        self.click_down = false;
        self.click_up = false;
        self.mouse_wheel = Vector2::new(0., 0.);
        self.input_buffer = vec![];
    }
}
