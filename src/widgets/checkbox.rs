use crate::{
    types::{Color, Rect, Vector2},
    Id, Layout, Ui,
};

pub struct Checkbox<'a> {
    id: Id,
    label: &'a str,
}

impl<'a> Checkbox<'a> {
    pub fn new(id: Id) -> Checkbox<'a> {
        Checkbox { id, label: "" }
    }

    pub fn label<'b>(self, label: &'b str) -> Checkbox<'b> {
        Checkbox { id: self.id, label }
    }

    pub fn ui(self, ui: &mut Ui, data: &mut bool) {
        let context = ui.get_active_window_context();

        let size = Vector2::new(
            context.window.cursor.area.w
                - context.global_style.margin * 2.
                - context.window.cursor.ident,
            19.,
        );
        let pos = context.window.cursor.fit(size, Layout::Vertical);

        let whole_area = Vector2::new(
            if self.label.is_empty() {
                size.x
            } else {
                size.x / 2.0
            },
            size.y,
        );
        let checkbox_area = Vector2::new(19., 19.);
        let checkbox_pos = Vector2::new(pos.x + whole_area.x / 2. - 19. / 2., pos.y);

        let hovered = Rect::new(
            checkbox_pos.x,
            checkbox_pos.y,
            checkbox_area.x,
            checkbox_area.y,
        )
        .contains(context.input.mouse_position);

        context.window.draw_commands.draw_rect(
            Rect::new(
                checkbox_pos.x,
                checkbox_pos.y,
                checkbox_area.x,
                checkbox_area.y,
            ),
            None,
            context
                .global_style
                .checkbox_background(context.focused, hovered),
        );

        if *data {
            context.window.draw_commands.draw_rect(
                Rect::new(
                    checkbox_pos.x + 3.,
                    checkbox_pos.y + 3.,
                    checkbox_area.x - 6.,
                    checkbox_area.y - 6.,
                ),
                None,
                context
                    .global_style
                    .checkbox_mark_background(context.focused),
            );
        }

        if hovered && context.input.click_up() {
            *data ^= true;
        }

        let context = ui.get_active_window_context();

        if self.label.is_empty() == false {
            context.window.draw_commands.draw_label(
                self.label,
                Vector2::new(pos.x + size.x / 2. + 5., pos.y + 2.),
                Color::from_rgba(0, 0, 0, 255),
            );
        }
    }
}

impl Ui {
    pub fn checkbox(&mut self, id: Id, label: &str, data: &mut bool) {
        Checkbox::new(id).label(label).ui(self, data)
    }
}
