use crate::{
    hash,
    types::{Rect, Vector2},
    widgets::Editbox,
    Id, Layout, Ui,
};
use std::ops::Range;

pub struct Slider<'a> {
    id: Id,
    label: &'a str,
    range: Range<f32>,
}

impl<'a> Slider<'a> {
    pub fn new(id: Id, range: Range<f32>) -> Slider<'a> {
        Slider {
            id,
            range,
            label: "",
        }
    }

    pub fn label<'b>(self, label: &'b str) -> Slider<'b> {
        Slider {
            id: self.id,
            range: self.range,
            label,
        }
    }

    pub fn ui(self, ui: &mut Ui, data: &mut f32) {
        let context = ui.get_active_window_context();

        let size = Vector2::new(
            context.window.cursor.area.w
                - context.global_style.margin * 2.
                - context.window.cursor.ident
                - context.window.cursor.x,
            19.,
        );
        let pos = context.window.cursor.fit(size, Layout::Vertical);

        let editbox_width = 50.;
        let label_width = 100.;
        let slider_width = size.x - editbox_width - label_width;
        let margin = 5.;

        let mut string = format!("{:1}", *data);
        Editbox::new(hash!(self.id, "editbox"), Vector2::new(50., size.y))
            .position(pos)
            .multiline(false)
            .ui(ui, &mut string);

        if let Ok(num) = string.parse::<f32>() {
            if num > self.range.end {
                *data = self.range.end;
            } else if num < self.range.start {
                *data = self.range.start;
            } else {
                *data = num;
            }
        }

        let context = ui.get_active_window_context();
        let dragging = context
            .storage_u32
            .entry(hash!(self.id, "dragging"))
            .or_insert(0);

        let slider_start_x = editbox_width + pos.x + margin;
        let data_pos = (*data - self.range.start) / (self.range.end - self.range.start)
            * slider_width
            + slider_start_x;

        let bar_rect = Rect::new(data_pos - 4., pos.y, 8., 20.);
        let hovered = bar_rect.contains(context.input.mouse_position);

        if hovered && context.input.is_mouse_down() {
            *dragging = 1;
            context.input.cursor_grabbed = true;
        }

        if *dragging == 1 && context.input.is_mouse_down == false {
            context.input.cursor_grabbed = false;
            *dragging = 0;
        }

        if *dragging == 1 {
            let mouse_position = ((context.input.mouse_position.x - slider_start_x) / slider_width)
                .min(1.)
                .max(0.);
            *data = self.range.start + (self.range.end - self.range.start) * mouse_position;
        }

        context.window.draw_commands.draw_line(
            Vector2::new(pos.x + editbox_width + margin, pos.y + size.y / 2.),
            Vector2::new(
                pos.x + editbox_width + slider_width + margin,
                pos.y + size.y / 2.,
            ),
            context.global_style.text(context.focused),
        );

        context.window.draw_commands.draw_rect(
            bar_rect,
            None,
            context.global_style.slider_bar(context.focused, hovered),
        );

        context.window.draw_commands.draw_label(
            self.label,
            Vector2::new(
                pos.x + editbox_width + slider_width + margin * 2.,
                pos.y + 2.,
            ),
            context.global_style.text(context.focused),
        );
    }
}

impl Ui {
    pub fn slider(&mut self, id: Id, label: &str, range: Range<f32>, data: &mut f32) {
        Slider::new(id, range).label(label).ui(self, data)
    }
}
