use crate::{Color, LabelParams, Vector2, Rect};

#[derive(Debug, Clone)]
pub enum DrawCommand {
    DrawLabel {
        position: Vector2,
        label: String,
        params: LabelParams,
    },
    DrawRect {
        rect: Rect,
        stroke: Option<Color>,
        fill: Option<Color>,
    },
    DrawLine {
        start: Vector2,
        end: Vector2,
        color: Color,
    },
    Clip {
        rect: Option<Rect>,
    },
}

#[derive(Debug)]
pub(crate) struct DrawList {
    pub commands: Vec<DrawCommand>,
    clipping_zone: Option<Rect>,
}

impl DrawList {
    pub fn new() -> DrawList {
        DrawList {
            commands: vec![],
            clipping_zone: None,
        }
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }

    pub fn add_command(&mut self, cmd: DrawCommand) {
        self.commands.push(cmd);
    }

    pub fn draw_label<T: Into<LabelParams>>(&mut self, label: &str, position: Vector2, params: T) {
        if self.clipping_zone.map_or(false, |clip| {
            !clip.overlaps(&Rect::new(position.x - 150., position.y - 25., 200., 50.))
        }) {
            return;
        }

        self.add_command(DrawCommand::DrawLabel {
            position,
            label: label.to_string(),
            params: params.into(),
        })
    }

    pub fn draw_rect<S, T>(&mut self, rect: Rect, stroke: S, fill: T)
    where
        S: Into<Option<Color>>,
        T: Into<Option<Color>>,
    {
        if self
            .clipping_zone
            .map_or(false, |clip| !clip.overlaps(&rect))
        {
            return;
        }

        self.add_command(DrawCommand::DrawRect {
            rect,
            stroke: stroke.into(),
            fill: fill.into(),
        })
    }

    pub fn draw_line<T: Into<Color>>(&mut self, start: Vector2, end: Vector2, color: T) {
        if self
            .clipping_zone
            .map_or(false, |clip| !clip.contains(start) && !clip.contains(end))
        {
            return;
        }

        self.add_command(DrawCommand::DrawLine {
            start,
            end,
            color: color.into(),
        });
    }

    pub fn clip<T: Into<Option<Rect>>>(&mut self, rect: T) {
        let rect = rect.into();

        self.clipping_zone = rect;

        self.add_command(DrawCommand::Clip { rect });
    }
}
