use crate::{
    hash,
    types::{Color, Rect, Vector2},
    widgets::Editbox,
    Id, Layout, Ui,
};
use std::num::NonZeroUsize;

struct State<T: MegaNum> {
    string_represents: T,
    string: String,
    before: String,
    drag: Option<Drag<T>>,
}

impl<T: MegaNum> Default for State<T> {
	fn default() -> Self {
		Self {
			string_represents: T::empty(),
			string: String::new(),
			before: String::new(),
			drag: None,
		}
	}
}

#[derive(Clone, Copy)]
struct Drag<T> {
    start_mouse: Vector2,
    start_data: T,
}

pub struct InputNum<'a, T: MegaNum> {
    id: Id,
    label: &'a str,
    size: Option<Vector2>,
    step: T,
}

impl<'a, T: MegaNum + 'static> InputNum<'a, T> {
    pub fn new(id: Id) -> InputNum<'a, T> {
        InputNum {
            id,
            size: None,
            label: "",
            step: T::default_step(),
        }
    }

    pub fn label<'b>(self, label: &'b str) -> InputNum<'b, T> {
        InputNum {
            label,
            id: self.id,
            size: self.size,
            step: self.step,
        }
    }

    pub fn size(self, size: Vector2) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }

    /// Ratio of pixels on the x-axis dragged to how much the value should be changed.
    /// Default for floating point numbers is `0.1`, for integers it's `1`.
    pub fn step(self, step: T) -> Self {
        Self { step, ..self }
    }

    pub fn ui(self, ui: &mut Ui, data: &mut T) {
        let context = ui.get_active_window_context();
        let state_hash = hash!(self.id, "input_float_state");
        let mut s: State<T> = std::mem::take(context.storage_any.get_or_default(state_hash));

        let size = self.size.unwrap_or_else(|| {
            Vector2::new(
                context.window.cursor.area.w
                    - context.global_style.margin * 2.
                    - context.window.cursor.ident,
                19.,
            )
        });
        let pos = context.window.cursor.fit(size, Layout::Vertical);

        let editbox_area = Vector2::new(
            if self.label.is_empty() {
                size.x
            } else {
                size.x / 2.0
            },
            size.y,
        );

        let mut editbox = Editbox::new(self.id, editbox_area)
            .position(pos)
            .multiline(false);

        let hovered = Rect::new(pos.x, pos.y, editbox_area.x, editbox_area.y)
            .contains(context.input.mouse_position);

        if hovered && context.input.is_mouse_down() {
            s.drag = Some(Drag {
                start_mouse: context.input.mouse_position,
                start_data: *data,
            });
            context.window.input_focus = Some(self.id);
            context.input.cursor_grabbed = true;
        }

        if let Some(drag) = s.drag {
            if context.input.is_mouse_down == false {
                s.drag = None;
                context.input.cursor_grabbed = false;
                if !hovered {
                    context.window.input_focus = None;
                } else {
					editbox = editbox.select_all();
                }
            }

            let mouse_delta = context.input.mouse_position.x - drag.start_mouse.x;
            *data = T::drag(drag.start_data, mouse_delta, self.step);
        }

        if s.string_represents != *data {
            s.string = data.to_string();
        }

        editbox.ui(ui, &mut s.string);

        if let Ok(n) = s
            .string
            .parse()
            .or_else(|e| if s.string.is_empty() { Ok(T::empty()) } else { Err(e) })
        {
            *data = n;
            s.string_represents = n;
            s.before = s.string.clone();
        } else {
            s.string = s.before.clone();
        }

        let context = ui.get_active_window_context();

        if self.label.is_empty() == false {
            context.window.draw_commands.draw_label(
                self.label,
                Vector2::new(pos.x + size.x / 2. + 5., pos.y + 2.),
                Color::from_rgba(0, 0, 0, 255),
            );
        }

        *context.storage_any.get_or_default(state_hash) = s;
    }
}

pub trait MegaNum: ToString + std::str::FromStr + PartialEq + Copy {
	fn default_step() -> Self;
	fn empty() -> Self;
	fn drag(start: Self, mouse_delta: f32, step: Self) -> Self;
}

macro_rules! mega_num_impls {
	( $($t:ident, $default_step:literal, $empty:literal ; )* ) => {$(
		impl MegaNum for $t {
			fn default_step() -> $t { $default_step }
			fn empty() -> $t { $empty }
			fn drag(start: $t, mouse_delta: f32, step: $t) -> $t {
				start + mouse_delta as $t * step
			}
		}
	)*}
}

mega_num_impls! {
	f32, 0.1, 0.0;
	f64, 0.1, 0.0;
	usize, 1, 0;
	u8, 1, 0;
	u16, 1, 0;
	u32, 1, 0;
	u64, 1, 0;
	u128, 1, 0;
	isize, 1, 0;
	i8, 1, 0;
	i16, 1, 0;
	i32, 1, 0;
	i64, 1, 0;
	i128, 1, 0;
}

mod non_zero {
	use super::*;
	use std::num::*;

	macro_rules! mega_num_non_zero_impls {
		( $($t:ident, $one:ident, $inside:ident;)* ) => {$(
			const $one: $t = unsafe { $t::new_unchecked(1) };
			impl MegaNum for $t {
				fn default_step() -> $t { $one }
				fn empty() -> $t { $one }
				fn drag(start: $t, mouse_delta: f32, step: $t) -> $t {
					$t::new(start.get() + mouse_delta as $inside * step.get()).unwrap_or($one)
				}
			}
		)*}
	}
	mega_num_non_zero_impls! {
		NonZeroUsize, NON_ZERO_USIZE_ONE, usize;
		NonZeroU8, NON_ZERO_U8_ONE, u8;
		NonZeroU16, NON_ZERO_U16_ONE, u16;
		NonZeroU32, NON_ZERO_U32_ONE, u32;
		NonZeroU64, NON_ZERO_U64_ONE, u64;
		NonZeroU128, NON_ZERO_U128_ONE, u128;
		NonZeroIsize, NON_ZERO_ISIZE_ONE, isize;
		NonZeroI8, NON_ZERO_I8_ONE, i8;
		NonZeroI16, NON_ZERO_I16_ONE, i16;
		NonZeroI32, NON_ZERO_I32_ONE, i32;
		NonZeroI64, NON_ZERO_I64_ONE, i64;
		NonZeroI128, NON_ZERO_I128_ONE, i128;
	}
}

impl Ui {
    pub fn input_num<T: MegaNum + 'static>(&mut self, id: Id, label: &str, data: &mut T) {
        InputNum::new(id).label(label).ui(self, data)
    }
}
