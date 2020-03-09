use super::Color;
use crate::math::{Position, Point, Scale, Region, Angle};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct SpriteDrawParams {
    pub(crate) region: Option<Region>,
    pub(crate) origin: Option<Point>,
    pub(crate) position: Option<Position>,
    pub(crate) rotation: Option<Angle>,
    pub(crate) scale: Option<Scale>,
    pub(crate) colors: Option<[Color; 4]>,
}

impl SpriteDrawParams {

    pub fn region(mut self, region: impl Into<Region>) -> Self {
        self.region = Some(region.into());
        self
    }

    pub fn origin(mut self, origin: impl Into<Point>) -> Self {
        self.origin = Some(origin.into());
        self
    }

    pub fn position(mut self, position: impl Into<Position>) -> Self {
        self.position = Some(position.into());
        self
    }

    pub fn rotation(mut self, angle: Angle) -> Self {
        self.rotation = Some(angle);
        self
    }

    pub fn scale(mut self, scale: impl Into<Scale>) -> Self {
        self.scale = Some(scale.into());
        self
    }

    pub fn colors(mut self, colors: [Color; 4]) -> Self {
        self.colors = Some(colors);
        self
    }

    pub fn color(mut self, color: impl Into<Color>) -> Self {
        let color = color.into();
        self.colors = Some([color, color, color, color]);
        self
    }

}
