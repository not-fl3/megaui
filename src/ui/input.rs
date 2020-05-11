use crate::types::Vector2;

#[derive(Clone, Debug)]
pub enum InputCharacter {
    Char(char),
    ControlCode {
	key_code: crate::input_handler::KeyCode,
	modifier_shift: bool
    }
}

#[derive(Default, Clone)]
pub struct Input {
    pub mouse_position: Vector2,
    pub is_mouse_down: bool,
    pub click_down: bool,
    pub click_up: bool,
    pub mouse_wheel: Vector2,
    pub input_buffer: Vec<InputCharacter>,
    pub cursor_grabbed: bool,
}

impl Input {
    pub fn clicked(&self) -> bool {
        self.is_mouse_down && self.cursor_grabbed == false
    }

    pub fn reset(&mut self) {
        self.click_down = false;
        self.click_up = false;
        self.mouse_wheel = Vector2::new(0., 0.);
        self.input_buffer = vec![];
    }
}
