pub trait InputHandler {
    fn mouse_down(&mut self, position: (f32, f32));
    fn mouse_up(&mut self, _: (f32, f32));
    fn mouse_wheel(&mut self, x: f32, y: f32);
    fn mouse_move(&mut self, position: (f32, f32));
}
