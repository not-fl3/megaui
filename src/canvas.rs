//! In-window drawing canvas for custom primitives like lines, rect and textures

use crate::ui::WindowContext;
use crate::{Color, Rect, Vector2, Layout};

pub struct DrawCanvas<'a> {
    pub(crate) context: WindowContext<'a>,
}

impl<'a> DrawCanvas<'a> {
    pub fn cursor(&self) -> Vector2 {
        let cursor = &self.context.window.cursor;
        Vector2::new(cursor.x, cursor.y)
            + Vector2::new(cursor.area.x as f32, cursor.area.y as f32)
            + cursor.scroll.scroll
    }

    pub fn request_space(&mut self, space: Vector2) -> Vector2 {
        let cursor = &mut self.context.window.cursor;

        cursor.fit(space, Layout::Vertical)
    }

    pub fn rect<S, T>(&mut self, rect: Rect, stroke: S, fill: T)
    where
        S: Into<Option<Color>>,
        T: Into<Option<Color>>,
    {
        self.context.register_click_intention(rect);

        self.context
            .window
            .draw_commands
            .draw_rect(rect, stroke, fill);
    }

    pub fn line(&mut self, start: Vector2, end: Vector2, color: Color) {
        self.context
            .window
            .draw_commands
            .draw_line(start, end, color);
    }

    pub fn image(&mut self, rect: Rect, texture: u32) {
        self.context.register_click_intention(rect);

        self.context
            .window
            .draw_commands
            .draw_raw_texture(rect, texture);
    }
}
