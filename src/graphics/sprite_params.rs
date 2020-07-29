use super::Color;
use crate::math::Region;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SpriteDrawParams {
    pub region: Option<Region>,
    pub colors: Option<[Color; 4]>,
}

impl SpriteDrawParams {

    pub fn region(mut self, region: impl Into<Region>) -> Self {
        self.region = Some(region.into());
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
