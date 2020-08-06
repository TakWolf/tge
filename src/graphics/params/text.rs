use super::Color;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum TextHorizontalGravity {
    Start,
    Center,
    End,
}

impl Default for TextHorizontalGravity {

    fn default() -> Self {
        Self::Start
    }

}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum TextVerticalGravity {
    Top,
    Middle,
    Bottom,
}

impl Default for TextVerticalGravity {

    fn default() -> Self {
        Self::Top
    }

}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct TextDrawParams {
    pub text_size: Option<f32>,
    pub char_spacing: Option<f32>,
    pub line_height: Option<f32>,
    pub line_spacing: Option<f32>,
    pub wrap_width: Option<f32>,
    pub wrap_height: Option<f32>,
    pub horizontal_gravity: Option<TextHorizontalGravity>,
    pub vertical_gravity: Option<TextVerticalGravity>,
    pub color: Option<Color>,
}

impl TextDrawParams {

    pub fn text_size(mut self, size: f32) -> Self {
        self.text_size = Some(size);
        self
    }

    pub fn char_spacing(mut self, spacing: f32) -> Self {
        self.char_spacing = Some(spacing);
        self
    }

    pub fn line_height(mut self, height: f32) -> Self {
        self.line_height = Some(height);
        self
    }

    pub fn line_spacing(mut self, spacing: f32) -> Self {
        self.line_spacing = Some(spacing);
        self
    }

    pub fn wrap_width(mut self, width: f32) -> Self {
        self.wrap_width = Some(width);
        self
    }

    pub fn wrap_height(mut self, height: f32) -> Self {
        self.wrap_height = Some(height);
        self
    }

    pub fn horizontal_gravity(mut self, gravity: TextHorizontalGravity) -> Self {
        self.horizontal_gravity = Some(gravity);
        self
    }

    pub fn vertical_gravity(mut self, gravity: TextVerticalGravity) -> Self {
        self.vertical_gravity = Some(gravity);
        self
    }

    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.color = Some(color.into());
        self
    }

}
