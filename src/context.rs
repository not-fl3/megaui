use crate::types::{Color, Point2, Rect, Vector2};

pub trait Context {
    fn draw_label<T: Into<Color>>(
        &mut self,
        label: &str,
        position: Point2,
        _: Option<()>,
        _: Option<()>,
        color: Option<T>,
    );
    fn measure_label(&mut self, label: &str, _: Option<()>) -> Vector2;
    fn draw_rect<S, T>(&mut self, rect: Rect, stroke: S, fill: T)
    where
        S: Into<Option<Color>>,
        T: Into<Option<Color>>;
    fn draw_line<T: Into<Color>>(&mut self, start: Point2, end: Point2, color: T);
    fn clip(&mut self, rect: Option<Rect>);
}
