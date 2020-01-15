mod context;
mod draw_list;
mod input_handler;
mod style;
mod types;
mod ui;

pub mod widgets;

pub use context::{Aligment, Context, LabelParams};
pub use input_handler::InputHandler;
pub use style::Style;
pub use types::{Color, Rect, Vector2};
pub use ui::{Drag, Layout, Ui};

pub type Id = u64;

#[macro_export]
macro_rules! hash {
    ($s:expr) => {{
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let id = $s;

        let mut s = DefaultHasher::new();
        id.hash(&mut s);
        s.finish()
    }};
    () => {{
        let id = concat!(file!(), line!(), column!());
        hash!(id)
    }};
    ($($s:expr),*) => {{
        let mut s: u128 = 0;
        $(s += hash!($s) as u128;)*
        hash!(s)
    }};
}
