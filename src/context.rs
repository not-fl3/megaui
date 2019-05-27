use crate::types::{Point2, Rect, RectAttr, Vector2, Color};

pub trait Context {
    fn draw_label(
        &mut self,
        label: &str,
        position: Point2,
        _: Option<()>,
        _: Option<()>,
        color: Option<&str>,
    );
    fn measure_label(&mut self, label: &str, _: Option<()>) -> Vector2;
    fn draw_rect(&mut self, rect: Rect, attrs: &[RectAttr]);
    fn draw_line<T: Into<Color>>(&mut self, start: Point2, end: Point2, color: T);
    fn clip(&mut self, rect: Option<Rect>);
}