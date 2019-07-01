use crate::types::Color;

use smart_default::SmartDefault;

#[derive(SmartDefault, Clone)]
pub struct Style {
    #[default(2.0)]
    pub margin: f32,
    #[default(14.0)]
    pub title_height: f32,

    #[default(10.0)]
    pub scroll_width: f32,
    #[default(3.)]
    pub scroll_multiplier: f32,

    #[default(Color::from_rgb(68, 68, 68))]
    pub window_border_focused: Color,
    #[default(Color::from_rgba(102, 102, 102, 127))]
    pub window_border_inactive: Color,

    #[default(Color::from_rgba(238, 238, 238, 255))]
    pub window_background_focused: Color,
    #[default(Color::from_rgba(238, 238, 238, 128))]
    pub window_background_inactive: Color,

    #[default(Color::from_rgba(170, 170, 170, 235))]
    pub scrollbar_background_focused_clicked: Color,
    #[default(Color::from_rgba(180, 180, 180, 235))]
    pub scrollbar_background_focused_hovered: Color,
    #[default(Color::from_rgba(204, 204, 204, 235))]
    pub scrollbar_background_focused: Color,
    #[default(Color::from_rgba(204, 204, 204, 128))]
    pub scrollbar_background_focused_inactive: Color,

    #[default(Color::from_rgba(102, 102, 102, 128))]
    pub inactive_title: Color,
    #[default(Color::from_rgba(0, 0, 0, 255))]
    pub focused_title: Color,

    #[default(Color::from_rgba(0, 0, 0, 255))]
    pub focused_text: Color,
    #[default(Color::from_rgba(102, 102, 102, 127))]
    pub inactive_text: Color,

    #[default(3.)]
    pub margin_button: f32,
    #[default(Color::from_rgba(204, 204, 204, 235))]
    pub button_background_focused: Color,
    #[default(Color::from_rgba(170, 170, 170, 235))]
    pub button_background_focused_hovered: Color,
    #[default(Color::from_rgba(187, 187, 187, 255))]
    pub button_background_focused_clicked: Color,
    #[default(Color::from_rgba(204, 204, 204, 127))]
    pub button_background_inactive: Color,

    #[default(Color::from_rgba(34, 153, 34, 68))]
    pub group_border_focused_hovered: Color,
    #[default(Color::from_rgba(34, 34, 34, 68))]
    pub group_border_focused: Color,
    #[default(Color::from_rgba(34, 34, 255, 255))]
    pub group_border_focused_highlight: Color,
    #[default(Color::from_rgba(17, 136, 17, 34))]
    pub group_border_inactive_hovered: Color,
    #[default(Color::from_rgba(17, 17, 17, 34))]
    pub group_border_inactive: Color,
}

impl Style {
    pub fn window_border(&self, focused: bool) -> Color {
        if focused {
            self.window_border_focused
        } else {
            self.window_border_inactive
        }
    }

    pub fn background(&self, focused: bool) -> Color {
        if focused {
            self.window_background_focused
        } else {
            self.window_background_inactive
        }
    }

    pub fn scroll_bar_handle(&self, focused: bool, hovered: bool, clicked: bool) -> Color {
        if focused {
            if clicked {
                self.scrollbar_background_focused_clicked
            } else if hovered {
                self.scrollbar_background_focused_hovered
            } else {
                self.scrollbar_background_focused
            }
        } else {
            self.scrollbar_background_focused_inactive
        }
    }

    pub fn title(&self, focused: bool) -> Color {
        if focused {
            self.focused_title
        } else {
            self.inactive_title
        }
    }

    pub fn text(&self, focused: bool) -> Color {
        if focused {
            self.focused_text
        } else {
            self.inactive_text
        }
    }

    pub fn button_background(&self, focused: bool, hovered: bool, clicked: bool) -> Color {
        if focused {
            if clicked {
                self.button_background_focused_clicked
            } else if hovered {
                self.button_background_focused_hovered
            } else {
                self.button_background_focused
            }
        } else {
            self.button_background_inactive
        }
    }

    pub fn drag_border(&self, focused: bool, hovered: bool, highlight: bool) -> Color {
        if focused {
            if hovered {
                self.group_border_focused_hovered
            } else {
                if highlight {
                    self.group_border_focused_highlight
                } else {
                    self.group_border_focused
                }
            }
        } else {
            if hovered {
                self.group_border_inactive_hovered
            } else {
                self.group_border_inactive
            }
        }
    }

    pub fn tabbar_background(
        &self,
        focused: bool,
        selected: bool,
        hovered: bool,
        clicked: bool,
    ) -> Color {
        if focused {
            if clicked {
                self.button_background_focused_clicked
            } else if hovered {
                self.button_background_focused
            } else {
                if selected {
                    self.button_background_focused_hovered
                } else {
                    self.button_background_inactive
                }
            }
        } else {
            self.button_background_inactive
        }
    }
}
