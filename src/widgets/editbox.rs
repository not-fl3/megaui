use crate::{
    hash,
    types::{Rect, Vector2},
    ui::InputCharacter,
    Id, Layout, Ui,
};

pub struct Editbox {
    id: Id,
    size: Vector2,
    pos: Option<Vector2>,
    line_height: f32,
}

const LEFT_MARGIN: f32 = 2.;

fn current_cursor_x_postion(text: &str, mut cursor: u32) -> u32 {
    let mut line_position = 0;
    while cursor > 0 && text.chars().nth(cursor as usize - 1).unwrap_or('x') != '\n' {
        cursor -= 1;
        line_position += 1;
    }
    line_position
}

impl Editbox {
    pub fn new(id: Id, size: Vector2) -> Editbox {
        Editbox {
            id,
            size,
            pos: None,
            line_height: 14.0,
        }
    }

    pub fn position(self, pos: Vector2) -> Editbox {
        Editbox {
            pos: Some(pos),
            ..self
        }
    }

    pub fn line_height(self, line_height: f32) -> Self {
        Self {
            line_height,
            ..self
        }
    }

    pub fn ui(self, ui: &mut Ui, text: &mut String) {
        let context = ui.get_active_window_context();

        let cursor = context.storage.entry(hash!(self.id, "cursor")).or_insert(0);

        let input_focused = context.window.input_focused(self.id);
        if context.focused && input_focused {
            for character in context.input.input_buffer.drain(0..) {
                use crate::input_handler::KeyCode;

                match character {
                    InputCharacter::Char(character) => {
                        if character != 13 as char
                            && character != 10 as char
                            && character.is_ascii()
                        {
                            text.insert(*cursor as usize, character);
                            *cursor += 1;
                        }
                    }
                    InputCharacter::ControlCode(code) => match code {
                        KeyCode::Enter => {
                            text.insert(*cursor as usize, '\n');
                            *cursor += 1;
                        }
                        KeyCode::Backspace => {
                            if *cursor > 0 {
                                text.remove(*cursor as usize - 1);
                                *cursor -= 1;
                            }
                        }
                        KeyCode::Delete => {
                            if *cursor < text.len() as u32 && text.len() != 0 {
                                text.remove(*cursor as usize);
                            }
                        }
                        KeyCode::Right => {
                            if *cursor < text.len() as u32 {
                                *cursor += 1;
                            }
                        }
                        KeyCode::Left => {
                            if *cursor > 0 {
                                *cursor -= 1;
                            }
                        }
                        KeyCode::Home => {
                            let line_position = current_cursor_x_postion(&text, *cursor);
                            *cursor -= line_position;
                        }
                        KeyCode::End => {
                            while *cursor < text.len() as u32
                                && text.chars().nth(*cursor as usize).unwrap_or('x') != '\n'
                            {
                                *cursor += 1;
                            }
                        }
                        KeyCode::Up => {
                            let line_position = current_cursor_x_postion(&text, *cursor);
                            *cursor -= line_position;
                            if *cursor != 0 {
                                *cursor -= 1;
                                let new_line_position = current_cursor_x_postion(&text, *cursor);
                                *cursor -= new_line_position;
                                *cursor += line_position.min(new_line_position);
                            }
                        }
                        KeyCode::Down => {
                            let line_position = current_cursor_x_postion(&text, *cursor);
                            while *cursor < text.len() as u32
                                && text.chars().nth(*cursor as usize).unwrap_or('x') != '\n'
                            {
                                *cursor += 1;
                            }
                            if text.len() != 0 && *cursor < text.len() as u32 - 1 {
                                *cursor += 1;
                                for _ in 0..line_position {
                                    if text.chars().nth(*cursor as usize).unwrap_or('x') == '\n'
                                        || *cursor == text.len() as u32 - 1
                                    {
                                        break;
                                    }
                                    *cursor += 1;
                                }
                            }
                        }
                    },
                }
            }
        }

        let color = context.global_style.text(context.focused);
        let pos = self
            .pos
            .unwrap_or_else(|| context.window.cursor.fit(self.size, Layout::Vertical));

        let rect = Rect::new(pos.x, pos.y, self.size.x, self.size.y);

        if context.input.is_mouse_down && rect.contains(context.input.mouse_position) {
            context.window.input_focus = Some(self.id);
        }

        // draw rect in parent window

        context.window.draw_commands.draw_rect(
            rect,
            context.global_style.editbox_background(context.focused),
            None,
        );

        // start child window for nice scroll inside the rect

        let parent = ui.get_active_window_context();
        parent.window.childs.push(self.id);
        let parent_id = Some(parent.window.id);

        let mut context = ui.begin_window(self.id, parent_id, pos, self.size, false);

        let size = Vector2::new(150., self.line_height * text.split('\n').count() as f32);

        let pos = context
            .window
            .cursor
            .fit(size, Layout::Free(Vector2::new(5., 5.)));

        context
            .window
            .draw_commands
            .clip(context.window.content_rect());

        context.scroll_area();

        let cursor = *context.storage.entry(hash!(self.id, "cursor")).or_insert(0);

        let mut x = LEFT_MARGIN;
        let mut y = 0.;

        for n in 0..text.len() + 1 {
            let character = text.chars().nth(n).unwrap_or(' ');
            if n == cursor as usize {
                context.window.draw_commands.draw_rect(
                    Rect::new(pos.x + x, pos.y + y - 2., 2., 13.),
                    context
                        .global_style
                        .editbox_cursor(context.focused, input_focused),
                    None,
                );
            }
            let mut advance = 0.;
            if character != '\n' {
                advance = context
                    .window
                    .draw_commands
                    .draw_character(character, pos + Vector2::new(x, y), color)
                    .unwrap_or(0.);
            }

            if context.input.is_mouse_down {
                let cursor_on_current_line =
                    (context.input.mouse_position.y - (pos.y + y + self.line_height / 2.)).abs()
                        < self.line_height / 2.;
                let line_end = character == '\n';
                let cursor_after_line_end = context.input.mouse_position.x > (pos.x + x);
                let clickable_character = character != '\n';
                let cursor_on_character =
                    (context.input.mouse_position.x - (pos.x + x)).abs() < advance / 2.;
                let last_character = n == text.len();
                let cursor_below_line =
                    (context.input.mouse_position.y - (pos.y + y + self.line_height)) > 0.;

                if (cursor_on_current_line && line_end && cursor_after_line_end)
                    || (cursor_on_current_line && clickable_character && cursor_on_character)
                    || (last_character && cursor_below_line)
                {
                    let cursor = context.storage.entry(hash!(self.id, "cursor")).or_insert(0);

                    *cursor = n as u32;
                }
            }

            x += advance;
            if character == '\n' {
                y += self.line_height;
                x = LEFT_MARGIN;
            }
        }

        let context = ui.get_active_window_context();

        context.window.draw_commands.clip(None);

        ui.end_window();
    }
}

impl Ui {
    pub fn editbox(&mut self, id: Id, size: Vector2, text: &mut String) {
        Editbox::new(id, size).ui(self, text)
    }
}
