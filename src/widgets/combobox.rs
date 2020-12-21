use crate::Color;
use crate::{
    hash,
    types::{Rect, Vector2},
    Id, Layout, Ui,
};

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

    pub fn ui(self, ui: &mut Ui, data: &mut usize) -> usize {
        let context = ui.get_active_window_context();

        let size = Vector2::new(
            context.window.cursor.area.w
                - context.global_style.margin * 2.
                - context.window.cursor.ident,
            19.,
        );
        let pos = context.window.cursor.fit(size, Layout::Vertical);

        let active_area_w = size.x / 2.;
        let triangle_area_w = 19.;

        let clickable_rect = Rect::new(pos.x, pos.y, active_area_w, size.y);
        let hovered = clickable_rect.contains(context.input.mouse_position);

        let state = context
            .storage_any
            .get_or_default::<bool>(hash!(self.id, "combobox_state"));

        if context.window.was_active == false {
            *state = false;
        }
        context.window.draw_commands.draw_rect(
            clickable_rect,
            context.global_style.editbox_background(context.focused),
            None,
        );
        context.window.draw_commands.draw_label(
            self.variants[*data],
            Vector2::new(pos.x + 5., pos.y + 2.),
            Color::from_rgba(0, 0, 0, 255),
        );

        context.window.draw_commands.draw_rect(
            Rect::new(
                pos.x + active_area_w - triangle_area_w,
                pos.y,
                triangle_area_w,
                size.y,
            ),
            context.global_style.editbox_background(context.focused),
            None,
        );
        context.window.draw_commands.draw_triangle(
            Vector2::new(pos.x + active_area_w - triangle_area_w + 4.0, pos.y + 4.0),
            Vector2::new(pos.x + active_area_w - 4.0, pos.y + 4.0),
            Vector2::new(pos.x + active_area_w - triangle_area_w / 2.0, pos.y + 15.0),
            Color::new(0.7, 0.7, 0.7, 1.0),
        );

        context.window.draw_commands.draw_label(
            self.label,
            Vector2::new(pos.x + size.x / 2. + 5., pos.y + 2.),
            Color::from_rgba(0, 0, 0, 255),
        );

        if context.focused && hovered && context.input.click_up {
            *state ^= true;
        }

        let modal_size = Vector2::new(200.0, self.variants.len() as f32 * 20.0 + 20.0);
        let modal_rect = Rect::new(pos.x, pos.y, modal_size.x, modal_size.y);
        if *state
            && (context.input.escape
                || (modal_rect.contains(context.input.mouse_position) == false
                    && context.input.click_down))
        {
            *state = false;
        }

        if *state {
            let context = ui.begin_modal(hash!("combobox", self.id), pos, modal_size);

            let state = context
                .storage_any
                .get_or_default::<bool>(hash!(self.id, "combobox_state"));

            for (i, variant) in self.variants.iter().enumerate() {
                let rect = Rect::new(
                    pos.x + 5.0,
                    pos.y + i as f32 * 20.0 + 20.0,
                    active_area_w - 5.0,
                    20.0,
                );
                let hovered = rect.contains(context.input.mouse_position);

                context.window.draw_commands.draw_rect(
                    rect,
                    context
                        .global_style
                        .combobox_variant_border(hovered, *data == i),
                    context
                        .global_style
                        .combobox_variant_background(hovered, *data == i),
                );

                context.window.draw_commands.draw_label(
                    variant,
                    Vector2::new(pos.x + 7., pos.y + i as f32 * 20.0 + 20.0 + 2.0),
                    Color::from_rgba(0, 0, 0, 255),
                );

                if hovered && context.input.click_up {
                    *data = i;
                    *state = false;
                }
            }
            ui.end_modal();
        }

        *data
    }
}

impl Ui {
    pub fn combo_box<'a>(
        &mut self,
        id: Id,
        label: &str,
        variants: &[&str],
        data: impl Into<Option<&'a mut usize>>,
    ) -> usize {
        if let Some(r) = data.into() {
            ComboBox::new(id, variants).label(label).ui(self, r)
        } else {
            let data_id = hash!(id, "selected_variant");
            let mut selected_variant = { *self.get_any(data_id) };

            ComboBox::new(id, variants)
                .label(label)
                .ui(self, &mut selected_variant);

            *self.get_any(data_id) = selected_variant;

            selected_variant
        }
    }
}
