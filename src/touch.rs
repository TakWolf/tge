mod phase;

pub use phase::TouchPhase;

use crate::error::GameResult;
use crate::math::Position;
use std::collections::HashMap;

pub struct Touch {
    positions: HashMap<u64, Position>,
}

impl Touch {

    pub(crate) fn new(touch_config: TouchConfig) -> GameResult<Self> {
        Ok(Self {
            positions: HashMap::new(),
        })
    }

    pub(crate) fn handle_event(&mut self, id: u64, phase: TouchPhase, position: Position) {
        match phase {
            TouchPhase::Start | TouchPhase::Move => self.positions.insert(id, position),
            TouchPhase::End | TouchPhase::Cancel => self.positions.remove(&id),
        };
    }

    pub(crate) fn clear_states(&mut self) {}

    pub fn touches(&self) -> Vec<u64> {
        let mut touches = Vec::with_capacity(self.positions.len());
        for (id, _) in &self.positions {
            touches.push(*id);
        }
        touches.sort();
        touches
    }

    pub fn position(&self, id: u64) -> Option<Position> {
        self.positions.get(&id).map(|position| *position)
    }

}

#[derive(Debug, Clone)]
pub struct TouchConfig {}

impl TouchConfig {

    pub fn new() -> Self {
        Self {}
    }

}
