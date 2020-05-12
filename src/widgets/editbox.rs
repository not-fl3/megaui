use crate::{
    hash,
    types::{Rect, Vector2},
    ui::InputCharacter,
    Id, Layout, Ui,
};

pub struct Editbox<'a> {
    id: Id,
    size: Vector2,
    multiline: bool,
    filter: Option<&'a dyn Fn(char) -> bool>,
    pos: Option<Vector2>,
    line_height: f32,
}

enum ClickState {
    None,
    SelectingChars { selection_begin: u32 },
    SelectingWords { selected_word: (u32, u32) },
    SelectingLines { selected_line: (u32, u32) },
    Selected,
}
impl Default for ClickState {
    fn default() -> ClickState {
        ClickState::None
    }
}
#[derive(Default)]
struct EditboxState {
    cursor: u32,
    click_state: ClickState,
    clicks_counter: u32,
    current_click: u32,
    last_click: u32,
    selection: Option<(u32, u32)>,
}

impl EditboxState {
    fn in_selected_range(&self, cursor: u32) -> bool {
        match self.selection {
            Some((start, end)) if start < end => cursor >= start && cursor < end,
            Some((end, start)) => cursor >= start && cursor < end,
            _ => false,
        }
    }
    fn find_line_begin(&self, text: &str) -> u32 {
        let mut line_position = 0;
        let mut cursor_tmp = self.cursor;

        while cursor_tmp > 0 && text.chars().nth(cursor_tmp as usize - 1).unwrap_or('x') != '\n' {
            cursor_tmp -= 1;
            line_position += 1;
        }
        line_position
    }

    fn find_line_end(&self, text: &str) -> u32 {
        let mut cursor_tmp = self.cursor;
        while cursor_tmp < text.len() as u32
            && text.chars().nth(cursor_tmp as usize).unwrap_or('x') != '\n'
        {
            cursor_tmp += 1;
        }

        cursor_tmp - self.cursor
    }

    fn find_word_begin(&self, text: &str) -> u32 {
        let mut cursor_tmp = self.cursor;
        let mut offset = 0;

        while cursor_tmp > 0 {
            let current_char = text.chars().nth(cursor_tmp as usize - 1).unwrap_or(' ');
            if current_char == ' ' || current_char == '\n' {
                break;
            }
            offset += 1;
            cursor_tmp -= 1;
        }
        offset
    }

    fn find_word_end(&self, text: &str) -> u32 {
        let mut cursor_tmp = self.cursor;
        let mut offset = 0;

        while cursor_tmp < text.len() as u32 {
            let current_char = text.chars().nth(cursor_tmp as usize).unwrap_or(' ');
            if current_char == ' ' || current_char == '\n' {
                break;
            }
            cursor_tmp += 1;
            offset += 1;
        }
        offset
    }

    fn insert_character(&mut self, text: &mut String, character: char) {
        self.delete_selected(text);
        self.selection = None;
        text.insert(self.cursor as usize, character);
        self.cursor += 1;
    }

    fn delete_selected(&mut self, text: &mut String) {
        if let Some((start, end)) = self.selection {
            let min = start.min(end) as usize;
            let max = start.max(end) as usize;

            text.replace_range(min..max, "");

            self.cursor = start;
        }
        self.selection = None;
    }

    fn delete_next_character(&mut self, text: &mut String) {
        if self.cursor < text.len() as u32 && text.len() != 0 {
            text.remove(self.cursor as usize);
        }
    }

    fn delete_current_character(&mut self, text: &mut String) {
        if self.cursor > 0 {
            text.remove(self.cursor as usize - 1);
            self.cursor -= 1;
        }
    }

    fn move_cursor(&mut self, text: &String, dx: i32, shift: bool) {
        let start_cursor = self.cursor;
        let mut end_cursor = start_cursor;

        if self.cursor as i32 + dx <= text.len() as i32 && self.cursor as i32 + dx >= 0 {
            end_cursor = (self.cursor as i32 + dx) as u32;
            self.cursor = end_cursor;
        }

        if shift == false {
            self.selection = None;
        }
        if shift {
            match &mut self.selection {
                None => self.selection = Some((start_cursor, end_cursor)),
                Some((_, ref mut end)) => {
                    *end = end_cursor;
                }
            }
        }
    }

    fn move_cursor_within_line(&mut self, text: &String, dx: i32, shift: bool) {
        assert!(dx >= 0, "not implemented");

        for _ in 0..dx {
            if text.chars().nth(self.cursor as usize).unwrap_or('x') == '\n'
                || self.cursor == text.len() as u32
            {
                break;
            }
            self.move_cursor(text, 1, shift);
        }
    }

    fn deselect(&mut self) {
        self.selection = None;
    }

    fn select_word(&mut self, text: &str) -> (u32, u32) {
        let to_word_begin = self.find_word_begin(text) as u32;
        let to_word_end = self.find_word_end(text) as u32;
        let new_selection = (self.cursor - to_word_begin, self.cursor + to_word_end);

        self.selection = Some(new_selection);
        new_selection
    }

    fn select_line(&mut self, text: &str) -> (u32, u32) {
        let to_line_begin = self.find_line_begin(text) as u32;
        let to_line_end = self.find_line_end(text) as u32;
        let new_selection = (self.cursor - to_line_begin, self.cursor + to_line_end);

        self.selection = Some(new_selection);
        new_selection
    }

    fn click_down(&mut self, text: &str, cursor: u32) {
        self.current_click = cursor;

        if self.last_click == self.current_click {
            self.clicks_counter += 1;
            match self.clicks_counter % 3 {
                0 => {
                    self.deselect();
                    self.click_state = ClickState::None;
                }
                1 => {
                    let selected_word = self.select_word(text);
                    self.click_state = ClickState::SelectingWords { selected_word };
                }
                2 => {
                    let selected_line = self.select_line(text);
                    self.click_state = ClickState::SelectingLines { selected_line }
                }
                _ => unreachable!(),
            }
        } else {
            self.clicks_counter = 0;
            self.click_state = ClickState::SelectingChars {
                selection_begin: cursor,
            };
            self.selection = Some((cursor, cursor));
        }
    }

    fn click_move(&mut self, text: &str, cursor: u32) {
        self.cursor = cursor;

        if self.cursor != self.last_click {
            self.clicks_counter = 0;
        }

        match self.click_state {
            ClickState::SelectingChars { selection_begin } => {
                self.selection = Some((selection_begin, cursor));
            }
            ClickState::SelectingWords {
                selected_word: (from, to),
            } => {
                if cursor < from {
                    let word_begin = self.cursor - self.find_word_begin(text);
                    self.selection = Some((word_begin, to));
                    self.cursor = word_begin;
                } else if cursor > to {
                    let word_end = self.cursor + self.find_word_end(text);
                    self.selection = Some((from, word_end));
                    self.cursor = word_end;
                } else {
                    self.selection = Some((from, to));
                    self.cursor = to;
                }
            }
            ClickState::SelectingLines {
                selected_line: (from, to),
            } => {
                if cursor < from {
                    let line_begin = self.cursor - self.find_line_begin(text);
                    let line_end = self.cursor + self.find_line_end(text);
                    self.selection = Some((line_begin, to));
                    self.cursor = line_end;
                } else if cursor > to {
                    let line_end = self.cursor + self.find_line_end(text);
                    self.selection = Some((from, line_end));
                    self.cursor = line_end;
                } else {
                    self.selection = Some((from, to));
                    self.cursor = to;
                }
            }
            _ => {}
        }

        self.last_click = cursor;
    }

    fn click_up(&mut self, _text: &str) {
        self.click_state = ClickState::Selected;
        if let Some((from, to)) = self.selection {
            if from == to {
                self.click_state = ClickState::None;
            }
        }
    }
}

const LEFT_MARGIN: f32 = 2.;

impl<'a> Editbox<'a> {
    pub fn new(id: Id, size: Vector2) -> Editbox<'a> {
        Editbox {
            id,
            size,
            filter: None,
            multiline: true,
            pos: None,
            line_height: 14.0,
        }
    }

    pub fn multiline(self, multiline: bool) -> Self {
        Editbox { multiline, ..self }
    }

    pub fn position(self, pos: Vector2) -> Self {
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

    pub fn filter<'b>(self, filter: &'b dyn Fn(char) -> bool) -> Editbox<'b> {
        Editbox {
            id: self.id,
            line_height: self.line_height,
            pos: self.pos,
            multiline: self.multiline,
            size: self.size,
            filter: Some(filter),
        }
    }

    fn apply_keyboard_input(
        &self,
        input_buffer: &mut Vec<InputCharacter>,
        text: &mut String,
        state: &mut EditboxState,
    ) {
        for character in input_buffer.drain(0..) {
            use crate::input_handler::KeyCode;

            match character {
                InputCharacter::Char(character) => {
                    if character != 13 as char
                        && character != 10 as char
                        && character.is_ascii()
                        && self.filter.as_ref().map_or(true, |f| f(character))
                    {
                        state.insert_character(text, character);
                    }
                }
                InputCharacter::ControlCode {
                    key_code: KeyCode::Enter,
                    ..
                } => {
		    if self.multiline {
			state.insert_character(text, '\n');
		    }
                }
                InputCharacter::ControlCode {
                    key_code: KeyCode::Backspace,
                    ..
                } => {
                    if state.selection.is_none() {
                        state.delete_current_character(text);
                    } else {
                        state.delete_selected(text);
                    }
                }
                InputCharacter::ControlCode {
                    key_code: KeyCode::Delete,
                    ..
                } => {
                    if state.selection.is_none() {
                        state.delete_next_character(text);
                    } else {
                        state.delete_selected(text);
                    }
                }
                InputCharacter::ControlCode {
                    key_code: KeyCode::Right,
                    modifier_shift,
                } => {
                    state.move_cursor(text, 1, modifier_shift);
                }
                InputCharacter::ControlCode {
                    key_code: KeyCode::Left,
                    modifier_shift,
                } => {
                    state.move_cursor(text, -1, modifier_shift);
                }
                InputCharacter::ControlCode {
                    key_code: KeyCode::Home,
                    modifier_shift,
                } => {
                    let to_line_begin = state.find_line_begin(&text) as i32;
                    state.move_cursor(text, -to_line_begin, modifier_shift);
                }
                InputCharacter::ControlCode {
                    key_code: KeyCode::End,
                    modifier_shift,
                } => {
                    let to_line_end = state.find_line_end(&text) as i32;
                    state.move_cursor(text, to_line_end, modifier_shift);
                }
                InputCharacter::ControlCode {
                    key_code: KeyCode::Up,
                    modifier_shift,
                } => {
                    let to_line_begin = state.find_line_begin(&text) as i32;
                    state.move_cursor(text, -to_line_begin, modifier_shift);
                    if state.cursor != 0 {
                        state.move_cursor(text, -1, modifier_shift);
                        let new_to_line_begin = state.find_line_begin(&text) as i32;
                        let offset = to_line_begin.min(new_to_line_begin) - new_to_line_begin;
                        state.move_cursor(text, offset, modifier_shift);
                    }
                }
                InputCharacter::ControlCode {
                    key_code: KeyCode::Down,
                    modifier_shift,
                } => {
                    let to_line_begin = state.find_line_begin(&text) as i32;
                    let to_line_end = state.find_line_end(&text) as i32;

                    state.move_cursor(text, to_line_end, modifier_shift);
                    if text.len() != 0 && state.cursor < text.len() as u32 - 1 {
                        state.move_cursor(text, 1, modifier_shift);
                        state.move_cursor_within_line(text, to_line_begin, modifier_shift);
                    }
                }
            }
        }
    }

    pub fn ui(self, ui: &mut Ui, text: &mut String) {
        let context = ui.get_active_window_context();

        let mut state = context
            .storage_any
            .get_or_default::<EditboxState>(hash!(self.id, "cursor"));

        // in case the string was updated outside of editbox
        if state.cursor > text.len() as u32 {
            state.cursor = text.len() as u32;
        }

        let input_focused = context.window.input_focused(self.id);

        // reset selection state when lost focus
        if context.focused == false || input_focused == false {
            state.deselect();
            state.clicks_counter = 0;
        }

        if context.focused && input_focused {
            self.apply_keyboard_input(&mut context.input.input_buffer, text, &mut state);
        }

        let color = context.global_style.text(context.focused);
        let pos = self
            .pos
            .unwrap_or_else(|| context.window.cursor.fit(self.size, Layout::Vertical));

        let rect = Rect::new(pos.x, pos.y, self.size.x, self.size.y);

        if context.input.click_down() && rect.contains(context.input.mouse_position) {
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

        context.scroll_area();

        context
            .window
            .draw_commands
            .clip(context.window.content_rect());

        let state = context
            .storage_any
            .get_or_default::<EditboxState>(hash!(self.id, "cursor"));

        let mut x = LEFT_MARGIN;
        let mut y = 0.;

        for n in 0..text.len() + 1 {
            let character = text.chars().nth(n).unwrap_or(' ');
            if n == state.cursor as usize {
                context.window.draw_commands.draw_rect(
                    Rect::new(pos.x + x, pos.y + y - 2., 2., 13.),
                    context
                        .global_style
                        .editbox_cursor(context.focused, input_focused),
                    None,
                );
            }
            let mut advance = 1.5; // 1.5 - hack to make cursor on newlines visible
            if character != '\n' {
                advance = context
                    .window
                    .draw_commands
                    .draw_character(character, pos + Vector2::new(x, y), color)
                    .unwrap_or(0.);
            }
            if state.in_selected_range(n as u32) {
                let pos = pos + Vector2::new(x, y);

                context.window.draw_commands.draw_rect(
                    Rect::new(pos.x, pos.y - 2., advance, 13.),
                    None,
                    context.global_style.selection_background(context.focused),
                );
            }

            if context.input.is_mouse_down() && input_focused {
                let cursor_on_current_line =
                    (context.input.mouse_position.y - (pos.y + y + self.line_height / 2.)).abs()
                        < self.line_height / 2.;
                let line_end = character == '\n' || n == text.len();
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
                    if context.input.click_down() {
                        state.click_down(text, n as u32);
                    } else {
                        state.click_move(text, n as u32);
                    }
                }
            }

            x += advance;
            if character == '\n' && self.multiline {
                y += self.line_height;
                x = LEFT_MARGIN;
            }
        }

        if context.input.click_up() && input_focused {
            state.click_up(text);
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
