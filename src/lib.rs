//! GUI crate originally intended for use with macroquad.
//! The API is inspired largely by imgui.

mod canvas;
mod clipboard;
mod draw_command;
mod draw_list;
mod hash;
mod input_handler;
mod style;
mod types;
mod ui;

pub mod widgets;

pub use clipboard::ClipboardObject;
pub use draw_list::{DrawList, Vertex};
pub use input_handler::{InputHandler, KeyCode};
pub use style::Style;
pub use types::{Color, Rect, Vector2};
pub use ui::{Drag, Id, Layout, Ui};
