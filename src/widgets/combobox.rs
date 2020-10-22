use crate::Color;
use crate::{
    types::Rect,
    hash,
    Id, Layout, Ui,
};
use glam::Vec2;

pub struct ComboBox<'a, 'b, 'c> {
    id: Id,
    label: &'a str,
    variants: &'b [&'c str],
}

impl<'a, 'b, 'c> ComboBox<'a, 'b, 'c> {
    pub fn new(id: Id, variants: &'b [&'c str]) -> ComboBox<'a, 'b, 'c> {
        ComboBox {
            id,
            label: "",
            variants,
        }
    }

    pub fn label<'x>(self, label: &'x str) -> ComboBox<'x, 'b, 'c> {
        ComboBox {
            id: self.id,
            variants: self.variants,
            label,
        }
    }

    pub fn ui(self, ui: &mut Ui) -> usize {
        let context = ui.get_active_window_context();

        let size = Vec2::new(
            context.window.cursor.area.w
                - context.global_style.margin * 2.
                - context.window.cursor.ident,
            19.,
        );
        let pos = context.window.cursor.fit(size, Layout::Vertical);

        let active_area_w = size.x() / 2.;
        let triangle_area_w = 19.;

        let clickable_rect = Rect::new(pos, Vec2::new(active_area_w, size.y()));
        let hovered = clickable_rect.contains(context.input.mouse_position);

        let (ref mut state, ref mut selection) = context
            .storage_any
            .get_or_default::<(bool, usize)>(hash!(self.id, "combobox_state"));

        if context.window.was_active == false {
            *selection = 0;
            *state = false;
        }
        context.window.draw_commands.draw_rect(
            clickable_rect,
            context.global_style.editbox_background(context.focused),
            None,
        );
        context.window.draw_commands.draw_label(
            self.variants[*selection],
            pos + Vec2::new(5.0, 2.0),
            Color::from_rgba(0, 0, 0, 255),
        );

        context.window.draw_commands.draw_rect(
            Rect::new(
                pos + Vec2::new(active_area_w - triangle_area_w, 0.0),
                Vec2::new(triangle_area_w, size.y()),
            ),
            context.global_style.editbox_background(context.focused),
            None,
        );
        context.window.draw_commands.draw_triangle(
            pos + Vec2::new(active_area_w - triangle_area_w + 4.0, 4.0),
            pos + Vec2::new(active_area_w - 4.0, 4.0),
            pos + Vec2::new(active_area_w - triangle_area_w / 2.0, 15.0),
            Color::new(0.7, 0.7, 0.7, 1.0),
        );

        context.window.draw_commands.draw_label(
            self.label,
            pos + Vec2::new(size.x() / 2.0 + 5.0, 2.0),
            Color::from_rgba(0, 0, 0, 255),
        );

        if context.focused && hovered && context.input.click_up {
            *state ^= true;
        }

        if *state {
            let context = ui.begin_modal(
                hash!("combobox", self.id),
                pos,
                Vec2::new(200.0, self.variants.len() as f32 * 20.0 + 20.0),
            );

            let (ref mut state, ref mut selection) = context
                .storage_any
                .get_or_default::<(bool, usize)>(hash!(self.id, "combobox_state"));

            for (i, variant) in self.variants.iter().enumerate() {
                let rect = Rect::new(
                    pos + Vec2::new(5.0, i as f32 * 20.0 + 20.0),
                    Vec2::new(active_area_w - 5.0, 20.0),
                );
                let hovered = rect.contains(context.input.mouse_position);

                context.window.draw_commands.draw_rect(
                    rect,
                    context
                        .global_style
                        .combobox_variant_border(hovered, *selection == i),
                    context
                        .global_style
                        .combobox_variant_background(hovered, *selection == i),
                );

                context.window.draw_commands.draw_label(
                    variant,
                    pos + Vec2::new(7.0, i as f32 * 20.0 + 20.0 + 2.0),
                    Color::from_rgba(0, 0, 0, 255),
                );

                if hovered && context.input.click_up {
                    *selection = i;
                    *state = false;
                }
            }
            ui.end_modal();
        }

        let context = ui.get_active_window_context();

        let (_, ref mut selection) = context
            .storage_any
            .get_or_default::<(bool, usize)>(hash!(self.id, "combobox_state"));

        *selection
    }
}

impl Ui {
    pub fn combo_box(&mut self, id: Id, label: &str, variants: &[&str]) -> usize {
        ComboBox::new(id, variants).label(label).ui(self)
    }
}
