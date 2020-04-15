use crate::{Color, Rect, Vector2};

#[derive(Debug, Clone)]
pub(crate) enum DrawCommand {
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
    DrawRawTexture {
        position: Vector2,
        size: Vector2,
        texture: u32,
    },
    Clip {
        rect: Option<Rect>,
    },
}

impl DrawCommand {
    pub fn offset(&self, offset: Vector2) -> DrawCommand {
        match self.clone() {
            DrawCommand::DrawLabel {
                position,
                label,
                params,
            } => DrawCommand::DrawLabel {
                position: position + offset,
                label,
                params,
            },
            DrawCommand::DrawRawTexture {
                position,
                size,
                texture,
            } => DrawCommand::DrawRawTexture {
                position: position + offset,
                size,
                texture,
            },
            DrawCommand::DrawRect { rect, stroke, fill } => DrawCommand::DrawRect {
                rect: rect.offset(offset),
                stroke,
                fill,
            },
            DrawCommand::DrawLine { start, end, color } => DrawCommand::DrawLine {
                start: start + offset,
                end: end + offset,
                color,
            },
            DrawCommand::Clip { rect } => DrawCommand::Clip {
                rect: rect.map(|rect| rect.offset(offset)),
            },
        }
    }
}

#[derive(Debug)]
pub(crate) struct CommandsList {
    pub commands: Vec<DrawCommand>,
    clipping_zone: Option<Rect>,
}

impl CommandsList {
    pub fn new() -> CommandsList {
        CommandsList {
            commands: vec![],
            clipping_zone: None,
        }
    }

    pub fn clear(&mut self) {
        self.commands.clear();
        self.clipping_zone = None;
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

    pub fn draw_raw_texture(&mut self, texture: u32, position: Vector2, size: Vector2) {
        if self.clipping_zone.map_or(false, |clip| {
            !clip.overlaps(&Rect::new(position.x, position.y, size.x, size.y))
        }) {
            return;
        }

        self.add_command(DrawCommand::DrawRawTexture {
            position,
            size,
            texture,
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

#[derive(Clone, Debug)]
pub enum Aligment {
    Left,
    Center,
}

impl Default for Aligment {
    fn default() -> Aligment {
        Aligment::Left
    }
}

#[derive(Clone, Debug)]
pub struct LabelParams {
    pub color: Color,
    pub aligment: Aligment,
}

impl Default for LabelParams {
    fn default() -> LabelParams {
        LabelParams {
            color: Color::new(0., 0., 0., 1.),
            aligment: Aligment::default(),
        }
    }
}

impl From<Option<Color>> for LabelParams {
    fn from(color: Option<Color>) -> LabelParams {
        LabelParams {
            color: color.unwrap_or(Color::new(0., 0., 0., 1.)),
            ..Default::default()
        }
    }
}
impl From<Color> for LabelParams {
    fn from(color: Color) -> LabelParams {
        LabelParams {
            color,
            ..Default::default()
        }
    }
}
impl From<(Color, Aligment)> for LabelParams {
    fn from((color, aligment): (Color, Aligment)) -> LabelParams {
        LabelParams { color, aligment }
    }
}
