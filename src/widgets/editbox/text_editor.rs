#[derive(Debug)]
pub enum ClickState {
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
pub const DOUBLE_CLICK_TIME: f32 = 0.5;

#[derive(Default)]
pub struct EditboxState {
    pub cursor: u32,
    pub click_state: ClickState,
    pub clicks_counter: u32,
    pub current_click: u32,
    pub last_click_time: f32,
    pub last_click: u32,
    pub selection: Option<(u32, u32)>,
}

impl EditboxState {
    pub fn in_selected_range(&self, cursor: u32) -> bool {
        match self.selection {
            Some((start, end)) if start < end => cursor >= start && cursor < end,
            Some((end, start)) => cursor >= start && cursor < end,
            _ => false,
        }
    }
    pub fn find_line_begin(&self, text: &str) -> u32 {
        let mut line_position = 0;
        let mut cursor_tmp = self.cursor;

        while cursor_tmp > 0 && text.chars().nth(cursor_tmp as usize - 1).unwrap_or('x') != '\n' {
            cursor_tmp -= 1;
            line_position += 1;
        }
        line_position
    }

    pub fn find_line_end(&self, text: &str) -> u32 {
        let mut cursor_tmp = self.cursor;
        while cursor_tmp < text.len() as u32
            && text.chars().nth(cursor_tmp as usize).unwrap_or('x') != '\n'
        {
            cursor_tmp += 1;
        }

        cursor_tmp - self.cursor
    }

    pub fn word_delimeter(character: char) -> bool {
        character == ' '
            || character == '('
            || character == ')'
            || character == ';'
            || character == '\"'
    }


    pub fn find_word_begin(&self, text: &str, cursor: u32) -> u32 {
        let mut cursor_tmp = cursor;
        let mut offset = 0;

        while cursor_tmp > 0 {
            let current_char = text.chars().nth(cursor_tmp as usize - 1).unwrap_or(' ');
            if Self::word_delimeter(current_char) || current_char == '\n' {
		break;
	    }
            offset += 1;
            cursor_tmp -= 1;
        }
        offset
    }

    pub fn find_word_end(&self, text: &str, cursor: u32) -> u32 {
        let mut cursor_tmp = cursor;
        let mut offset = 0;
	let mut space_skipping = false;

        while cursor_tmp < text.len() as u32 {
            let current_char = text.chars().nth(cursor_tmp as usize).unwrap_or(' ');
            if Self::word_delimeter(current_char) || current_char == '\n' {
		space_skipping = true;
            }
	    if space_skipping && Self::word_delimeter(current_char) == false {
		break;
	    }
            cursor_tmp += 1;
            offset += 1;
        }
        offset
    }

    pub fn insert_character(&mut self, text: &mut String, character: char) {
        self.delete_selected(text);
        self.selection = None;
        text.insert(self.cursor as usize, character);
        self.cursor += 1;
    }

    pub fn delete_selected(&mut self, text: &mut String) {
        if let Some((start, end)) = self.selection {
            let min = start.min(end) as usize;
            let max = start.max(end) as usize;

            text.replace_range(min..max, "");

            self.cursor = start;
        }
        self.selection = None;
    }

    pub fn delete_next_character(&mut self, text: &mut String) {
        if self.cursor < text.len() as u32 && text.len() != 0 {
            text.remove(self.cursor as usize);
        }
    }

    pub fn delete_current_character(&mut self, text: &mut String) {
        if self.cursor > 0 {
            text.remove(self.cursor as usize - 1);
            self.cursor -= 1;
        }
    }

    pub fn move_cursor_next_word(&mut self, text: &str, shift: bool) {
        let next_word = self.find_word_end(text, self.cursor + 1) + 1;
        self.move_cursor(text, next_word as i32, shift);
    }

    pub fn move_cursor_prev_word(&mut self, text: &str, shift: bool) {
        if self.cursor > 1 {
            let prev_word = self.find_word_begin(text, self.cursor - 1) + 1;
            self.move_cursor(text, -(prev_word as i32), shift);
        }
    }

    pub fn move_cursor(&mut self, text: &str, dx: i32, shift: bool) {
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

    pub fn move_cursor_within_line(&mut self, text: &String, dx: i32, shift: bool) {
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

    pub fn deselect(&mut self) {
        self.selection = None;
    }

    pub fn select_word(&mut self, text: &str) -> (u32, u32) {
        let to_word_begin = self.find_word_begin(text, self.cursor) as u32;
        let to_word_end = self.find_word_end(text, self.cursor) as u32;
        let new_selection = (self.cursor - to_word_begin, self.cursor + to_word_end);

        self.selection = Some(new_selection);
        new_selection
    }

    pub fn select_line(&mut self, text: &str) -> (u32, u32) {
        let to_line_begin = self.find_line_begin(text) as u32;
        let to_line_end = self.find_line_end(text) as u32;
        let new_selection = (self.cursor - to_line_begin, self.cursor + to_line_end);

        self.selection = Some(new_selection);
        new_selection
    }

    pub fn click_down(&mut self, time: f32, text: &str, cursor: u32) {
        self.current_click = cursor;

        if self.last_click == self.current_click && time - self.last_click_time < DOUBLE_CLICK_TIME
        {
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
            if let ClickState::None | ClickState::Selected = self.click_state {
                self.click_state = ClickState::SelectingChars {
                    selection_begin: cursor,
                };
                self.selection = Some((cursor, cursor));
            } else {
                self.click_state = ClickState::None;
                self.selection = None;
                self.cursor = cursor;
            }
        }

        self.last_click_time = time;
    }

    pub fn click_move(&mut self, text: &str, cursor: u32) {
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
                    let word_begin = self.cursor - self.find_word_begin(text, self.cursor);
                    self.selection = Some((word_begin, to));
                    self.cursor = word_begin;
                } else if cursor > to {
                    let word_end = self.cursor + self.find_word_end(text, self.cursor);
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

    pub fn click_up(&mut self, _text: &str) {
        self.click_state = ClickState::None;
        if let Some((from, to)) = self.selection {
            if from != to {
                self.click_state = ClickState::Selected;
            }
        }
    }
}
