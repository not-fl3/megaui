use cgmath::Point2;

pub trait InputHandler {
    fn mouse_down(&mut self, position: Point2<f32>);
    fn mouse_up(&mut self, _: Point2<f32>);
    fn mouse_wheel(&mut self, x: f32, y: f32);
    fn mouse_move(&mut self, position: Point2<f32>);
}
