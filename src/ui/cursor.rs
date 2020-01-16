use crate::types::{Rect, Vector2};

#[derive(Clone, Debug)]
pub struct Scroll {
    pub scroll: Vector2,
    pub dragging_x: bool,
    pub dragging_y: bool,
    pub rect: Rect,
    pub inner_rect: Rect,
    pub inner_rect_previous_frame: Rect,
    pub initial_scroll: Vector2,
}
impl Scroll {
    pub fn scroll_to(&mut self, y: f32) {
        self.rect.y = y
            .max(self.inner_rect_previous_frame.y)
            .min(self.inner_rect_previous_frame.h - self.rect.h + self.inner_rect_previous_frame.y);
    }

    pub fn update(&mut self) {
        self.rect.y =
            self.rect.y.max(self.inner_rect_previous_frame.y).min(
                self.inner_rect_previous_frame.h - self.rect.h + self.inner_rect_previous_frame.y,
            );
    }
}

#[derive(Debug)]
pub enum Layout {
    Vertical,
    Horizontal,
    Free(Vector2),
}

#[derive(Debug)]
pub struct Cursor {
    pub x: f32,
    pub y: f32,
    pub start_x: f32,
    pub start_y: f32,
    pub ident: f32,
    pub scroll: Scroll,
    pub area: Rect,
    pub margin: f32,
}

impl Cursor {
    pub fn new(area: Rect, margin: f32) -> Cursor {
        Cursor {
            margin: margin,
            x: margin,
            y: margin,
            ident: 0.,
            start_x: margin,
            start_y: margin,
            scroll: Scroll {
                rect: Rect::new(0., 0., area.w, area.h),
                inner_rect: Rect::new(0., 0., area.w, area.h),
                inner_rect_previous_frame: Rect::new(0., 0., area.w, area.h),
                scroll: Vector2::new(0., 0.),
                dragging_x: false,
                dragging_y: false,
                initial_scroll: Vector2::new(0., 0.),
            },
            area,
        }
    }

    pub fn reset(&mut self) {
        self.x = self.start_x;
        self.y = self.start_y;
        self.ident = 0.;
        self.scroll.inner_rect_previous_frame = self.scroll.inner_rect;
        self.scroll.inner_rect = Rect::new(0., 0., self.area.w, self.area.h);
    }

    pub fn fit(&mut self, size: Vector2, layout: Layout) -> Vector2 {
        let res;

        match layout {
            Layout::Horizontal => {
                if self.x + size.x < self.area.w as f32 - self.margin * 2. {
                    res = Vector2::new(self.x, self.y);
                } else {
                    self.x = self.margin;
                    self.y += size.y + self.margin; // TODO: not size.y, but previous row max y, which is currently unknown :(
                    res = Vector2::new(self.x, self.y);
                }
                self.x += size.x + self.margin;
            }
            Layout::Vertical => {
                res = Vector2::new(self.x, self.y);
                self.x = self.margin;
                self.y += size.y + self.margin
            }
            Layout::Free(point) => {
                res = point;
            }
        }
        self.scroll.inner_rect = self
            .scroll
            .inner_rect
            .combine_with(Rect::new(res.x, res.y, size.x, size.y));
        res + Vector2::new(self.area.x as f32, self.area.y as f32)
            + self.scroll.scroll
            + Vector2::new(self.ident, 0.)
    }
}
