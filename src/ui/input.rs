use crate::types::Vector2;

#[derive(Default, Clone)]
pub struct Input {
    pub mouse_position: Vector2,
    pub is_mouse_down: bool,
    pub click_down: bool,
    pub click_up: bool,
    pub mouse_wheel: Vector2,
}

impl Input {
    pub fn reset(&mut self) {
        self.click_down = false;
        self.click_up = false;
        self.mouse_wheel = Vector2::new(0., 0.);
    }
}
