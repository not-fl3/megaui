use crate::types::{Color, Point2, Rect};

#[derive(Clone, Debug)]
pub enum Aligment {
    Left,
    Center,
}

impl Default for Aligment {
    fn default() -> Aligment {
        Aligment::Left
    }
}

#[derive(Clone, Debug)]
pub struct LabelParams {
    pub color: Color,
    pub aligment: Aligment,
}

impl Default for LabelParams {
    fn default() -> LabelParams {
        LabelParams {
            color: Color::new(0., 0., 0., 1.),
            aligment: Aligment::default(),
        }
    }
}

impl From<Option<Color>> for LabelParams {
    fn from(color: Option<Color>) -> LabelParams {
        LabelParams {
            color: color.unwrap_or(Color::new(0., 0., 0., 1.)),
            ..Default::default()
        }
    }
}
impl From<Color> for LabelParams {
    fn from(color: Color) -> LabelParams {
        LabelParams {
            color,
            ..Default::default()
        }
    }
}
impl From<(Color, Aligment)> for LabelParams {
    fn from((color, aligment): (Color, Aligment)) -> LabelParams {
        LabelParams { color, aligment }
    }
}

pub trait Context {
    fn debug_log(&self, _: &str) {}

    fn draw_label<T: Into<LabelParams>>(&mut self, position: Point2, label: &str, params: T);
    fn draw_rect<S, T>(&mut self, rect: Rect, stroke: S, fill: T)
    where
        S: Into<Option<Color>>,
        T: Into<Option<Color>>;
    fn draw_line<T: Into<Color>>(&mut self, start: Point2, end: Point2, color: T);
    fn clip(&mut self, rect: Option<Rect>);
}
