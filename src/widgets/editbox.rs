use crate::{
    hash,
    types::{Color, Rect, Vector2},
    ui::InputCharacter,
    Id, Layout, Ui,
};

pub struct Editbox {
    id: Id,
    size: Vector2,
    line_height: f32,
}

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
            line_height: 14.0,
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

        if context.focused {
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
        let pos = context.window.cursor.fit(self.size, Layout::Vertical);

        context.window.draw_commands.draw_rect(
            Rect::new(pos.x, pos.y, self.size.x, self.size.y),
            context.global_style.editbox_background(context.focused),
            None,
        );

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

        let mut x = 10.;
        let mut y = 0.;

        for n in 0..text.len() + 1 {
            let character = text.chars().nth(n).unwrap_or(' ');
            if n == cursor as usize {
                context.window.draw_commands.draw_rect(
                    Rect::new(pos.x + x, pos.y + y, 2., 13.),
                    Color::new(0., 0., 0., 1.),
                    None,
                );
            }
            if character != '\n' {
                context.window.draw_commands.draw_label(
                    &character.to_string(),
                    pos + Vector2::new(x, y),
                    color,
                );
            }
            x += 10.;
            if character == '\n' {
                y += self.line_height;
                x = 10.;
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
