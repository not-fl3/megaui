use crate::{
    draw_list::{DrawCommand, DrawList},
    types::Rect,
    types::{Point2, Vector2},
    Context, Id, InputHandler, Style,
};

use rustc_hash::FxHashMap;

mod cursor;
mod input;

use cursor::Cursor;
use input::Input;

pub use cursor::Layout;

#[derive(Debug)]
pub(crate) struct Window {
    pub id: Id,
    pub parent: Option<Id>,
    pub visible: bool,
    pub was_active: bool,
    pub active: bool,
    pub title_height: f32,
    pub position: Point2,
    pub size: Vector2,
    pub draw_list: DrawList,
    pub cursor: Cursor,
    pub childs: Vec<Id>,
    pub want_close: bool,
}
impl Window {
    pub fn new(
        id: Id,
        parent: Option<Id>,
        position: Point2,
        size: Vector2,
        title_height: f32,
        margin: f32,
    ) -> Window {
        Window {
            id,
            position,
            size,
            title_height,
            parent,
            visible: true,
            was_active: false,
            active: false,
            draw_list: DrawList::new(),
            cursor: Cursor::new(
                Rect::new(
                    position.x,
                    position.y + title_height,
                    size.x,
                    size.y - title_height,
                ),
                margin,
            ),
            childs: vec![],
            want_close: false,
        }
    }

    pub fn top_level(&self) -> bool {
        self.parent.is_none()
    }

    pub fn full_rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.size.x, self.size.y)
    }

    pub fn content_rect(&self) -> Rect {
        Rect::new(
            self.position.x,
            self.position.y + self.title_height,
            self.size.x,
            self.size.y - self.title_height,
        )
    }

    pub fn set_position(&mut self, position: Point2) {
        self.position = position;
        self.cursor.area.x = position.x;
        self.cursor.area.y = position.y + self.title_height;
    }

    pub fn title_rect(&self) -> Rect {
        Rect::new(
            self.position.x,
            self.position.y,
            self.size.x,
            self.title_height,
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub enum DragState {
    Clicked(Point2),
    Dragging(Point2),
}

#[derive(Copy, Clone, Debug)]
pub enum Drag {
    No,
    Dragging,
    Dropped(Point2, Option<Id>),
}

pub struct Ui {
    input: Input,
    pub(crate) style: Style,
    frame: u64,

    moving: Option<(Id, Vector2)>,
    windows: FxHashMap<Id, Window>,
    windows_focus_order: Vec<Id>,

    storage: FxHashMap<Id, u32>,

    dragging: Option<(Id, DragState)>,
    drag_hovered: Option<Id>,
    active_window: Option<Id>,
    child_window_stack: Vec<Id>,
}

pub(crate) struct WindowContext<'a> {
    pub window: &'a mut Window,
    pub dragging: &'a mut Option<(Id, DragState)>,
    pub drag_hovered: &'a mut Option<Id>,
    pub storage: &'a mut FxHashMap<Id, u32>,
    pub global_style: &'a Style,
    pub input: &'a Input,
    pub focused: bool,
}

impl<'a> WindowContext<'a> {
    pub(crate) fn scroll_area(&mut self) {
        let inner_rect = self.window.cursor.scroll.inner_rect_previous_frame;
        let rect = self.window.content_rect();

        self.window.cursor.scroll.scroll = Vector2::new(
            -self.window.cursor.scroll.rect.x,
            -self.window.cursor.scroll.rect.y,
        );

        if inner_rect.h > rect.h {
            self.draw_vertical_scroll_bar(
                rect,
                Rect::new(
                    rect.x + rect.w - self.global_style.scroll_width,
                    rect.y,
                    self.global_style.scroll_width,
                    rect.h,
                ),
            );
        }

        self.window.cursor.scroll.update();
    }

    pub(crate) fn close(&mut self) {
        self.window.want_close = true;
    }

    fn draw_vertical_scroll_bar(&mut self, area: Rect, rect: Rect) {
        let mut scroll = &mut self.window.cursor.scroll;
        let inner_rect = scroll.inner_rect_previous_frame;
        let size = scroll.rect.h / inner_rect.h * rect.h;
        let pos = (scroll.rect.y - inner_rect.y) / inner_rect.h * rect.h;

        self.window.draw_list.draw_line(
            Point2::new(rect.x, rect.y),
            Point2::new(rect.x, rect.y + rect.h),
            self.global_style.window_border(self.focused),
        );

        let mut clicked = false;
        let mut hovered = false;
        let bar = Rect::new(rect.x + 1., rect.y + pos, rect.w - 1., size);
        let k = inner_rect.h / scroll.rect.h;
        if bar.contains(self.input.mouse_position) {
            hovered = true;
        }
        if hovered && self.input.click_down {
            scroll.dragging_y = true;
            scroll.initial_scroll.y = scroll.rect.y - self.input.mouse_position.y * k;
        }
        if self.input.is_mouse_down == false {
            scroll.dragging_y = false;
        }
        if scroll.dragging_y {
            clicked = true;
            scroll.scroll_to(self.input.mouse_position.y * k + scroll.initial_scroll.y);
        }

        if self.focused
            && area.contains(self.input.mouse_position)
            && self.input.mouse_wheel.y != 0.
        {
            scroll.scroll_to(
                scroll.rect.y + self.input.mouse_wheel.y * k * self.global_style.scroll_multiplier,
            );
        }

        self.window.draw_list.draw_rect(
            bar,
            None,
            self.global_style
                .scroll_bar_handle(self.focused, hovered, clicked),
        );
    }
}

impl InputHandler for Ui {
    fn mouse_down(&mut self, position: Point2) {
        self.input.is_mouse_down = true;
        self.input.click_down = true;
        self.input.mouse_position = position;

        for (n, window) in self.windows_focus_order.iter().enumerate() {
            let window = &self.windows[window];

            if window.was_active == false {
                continue;
            }

            if window.top_level() && window.title_rect().contains(position) {
                self.moving = Some((
                    window.id,
                    position - Point2::new(window.position.x, window.position.y),
                ));
            }

            if window.top_level() && window.full_rect().contains(position) {
                let window = self.windows_focus_order.remove(n);
                self.windows_focus_order.insert(0, window);
                return;
            }
        }
    }

    fn mouse_up(&mut self, _: Point2) {
        self.input.is_mouse_down = false;
        self.input.click_up = true;
        self.moving = None;
    }

    fn mouse_wheel(&mut self, x: f32, y: f32) {
        self.input.mouse_wheel = Vector2::new(x, y);
    }

    fn mouse_move(&mut self, position: Point2) {
        self.input.mouse_position = position;
        if let Some((id, orig)) = self.moving.as_ref() {
            self.windows
                .get_mut(id)
                .unwrap()
                .set_position(Point2::new(position.x - orig.x, position.y - orig.y));
        }
    }
}

impl Ui {
    pub fn new(style: Style) -> Ui {
        Ui {
            input: Input::default(),
            style,
            frame: 0,
            moving: None,
            windows: FxHashMap::default(),
            windows_focus_order: vec![],
            dragging: None,
            active_window: None,
            child_window_stack: vec![],
            drag_hovered: None,
            storage: FxHashMap::default(),
        }
    }

    pub(crate) fn begin_window(
        &mut self,
        id: Id,
        parent: Option<Id>,
        position: Point2,
        size: Vector2,
    ) -> WindowContext {
        if let Some(active_window) = self.active_window {
            self.child_window_stack.push(active_window);
        }
        self.active_window = Some(id);

        let focused = self.is_focused(id);
        let title_height = if parent.is_none() {
            self.style.title_height
        } else {
            0.
        };
        let margin = self.style.margin;
        let windows_focus_order = &mut self.windows_focus_order;

        let window = &mut *self.windows.entry(id).or_insert_with(|| {
            if parent.is_none() {
                windows_focus_order.push(id);
            }
            Window::new(id, parent, position, size, title_height, margin)
        });

        window.size = size;
        window.want_close = false;
        window.active = true;

        // top level windows are moveble, so we update their position only on the first frame
        // while the child windows are not moveble and should update their position each frame
        if parent.is_some() {
            window.set_position(position);
        }

        WindowContext {
            focused,
            window,
            input: &self.input,
            global_style: &self.style,
            dragging: &mut self.dragging,
            drag_hovered: &mut self.drag_hovered,
            storage: &mut self.storage,
        }
    }

    pub(crate) fn end_window(&mut self) {
        self.active_window = self.child_window_stack.pop();
    }

    pub(crate) fn get_active_window_context(&mut self) -> WindowContext {
        let active_window = self
            .active_window
            .expect("Rendering outside of window unsupported");
        let focused = self.is_focused(active_window);
        let window = self.windows.get_mut(&active_window).unwrap();

        WindowContext {
            window,
            focused,
            input: &self.input,
            global_style: &self.style,
            dragging: &mut self.dragging,
            drag_hovered: &mut self.drag_hovered,
            storage: &mut self.storage,
        }
    }

    pub fn is_mouse_over(&self, mouse_position: Point2) -> bool {
        for window in self.windows_focus_order.iter() {
            let window = &self.windows[window];
            if window.was_active == false {
                continue;
            }
            if window.full_rect().contains(mouse_position) {
                return true;
            }
        }
        false
    }

    pub fn active_window_focused(&self) -> bool {
        self.active_window.map_or(false, |wnd| self.is_focused(wnd))
    }

    pub fn is_dragging(&self) -> bool {
        self.dragging.is_some()
    }

    pub fn close_current_window(&mut self) {
        let mut context = self.get_active_window_context();
        context.close();
    }

    pub fn is_focused(&self, id: Id) -> bool {
        if let Some(focused_window) = self
            .windows_focus_order
            .iter()
            .find(|window| self.windows[window].was_active)
        {
            if id == *focused_window {
                return true;
            }
            if let Some(parent) = self.child_window_stack.get(0) {
                return *parent == *focused_window;
            }
        }
        return false;
    }

    pub fn new_frame(&mut self) {
        self.frame += 1;

        for (_, window) in &mut self.windows {
            window.draw_list.clear();
            window.cursor.reset();
            window.was_active = window.active;
            window.active = false;
            window.childs.clear();
        }
    }

    pub fn render<T: Context>(&mut self, context: &mut T) {
        for window in self.windows_focus_order.iter().rev() {
            let window = &self.windows[window];
            if window.was_active {
                self.render_window(window, context, Vector2::new(0., 0.));
            }
        }

        if let Some((id, DragState::Dragging(orig))) = self.dragging {
            let window = &self.windows[&id];

            self.render_window(window, context, self.input.mouse_position - orig);
        }

        self.end_frame();
    }

    fn render_window<T: Context>(&self, window: &Window, context: &mut T, offset: Vector2) {
        for cmd in &window.draw_list.commands {
            match cmd.clone() {
                DrawCommand::DrawLabel {
                    position,
                    label,
                    params,
                } => {
                    context.draw_label(position + offset, &label, params);
                }
                DrawCommand::DrawRect { rect, stroke, fill } => {
                    context.draw_rect(rect.offset(offset), stroke, fill);
                }
                DrawCommand::DrawLine { start, end, color } => {
                    context.draw_line(start + offset, end + offset, color);
                }
                DrawCommand::Clip { rect } => {
                    context.clip(rect.map(|rect| rect.offset(offset)));
                }
            }
        }

        for child in &window.childs {
            let child_window = &self.windows[child];
            if window.content_rect().overlaps(&child_window.full_rect()) {
                context.clip(Some(window.content_rect()));
                self.render_window(child_window, context, offset);
                context.clip(None);
            }
        }
    }

    pub fn end_frame(&mut self) {
        self.input.reset();
    }

    pub fn focus_window(&mut self, id: Id) {
        if let Some(n) = self.windows_focus_order.iter().position(|win| *win == id) {
            let window = self.windows_focus_order.remove(n);
            self.windows_focus_order.insert(0, window);
        }
    }

    pub fn move_window(&mut self, id: Id, position: Point2) {
        if let Some(window) = self.windows.get_mut(&id) {
            window.set_position(position);
        }
    }
}
