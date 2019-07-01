use crate::types::{Point2, Vector2};
use smart_default::SmartDefault;

#[derive(SmartDefault, Clone)]
pub struct Input {
    #[default(Point2::new(0., 0.))]
    pub mouse_position: Point2,
    pub is_mouse_down: bool,
    pub click_down: bool,
    pub click_up: bool,
    #[default(Vector2::new(0., 0.))]
    pub mouse_wheel: Vector2,
}

impl Input {
    pub fn reset(&mut self) {
        self.click_down = false;
        self.click_up = false;
        self.mouse_wheel = Vector2::new(0., 0.);
    }
}
