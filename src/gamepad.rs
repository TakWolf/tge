mod button;
mod axis;

pub use button::GamepadButton;
pub use axis::GamepadAxis;

use crate::error::{GameError, GameResult};
use gilrs::{Gilrs, GilrsBuilder};

pub struct Gamepad {
    gilrs: Gilrs,
}

impl Gamepad {

    pub(crate) fn new(gamepad_config: GamepadConfig) -> GameResult<Self> {
        let gilrs = GilrsBuilder::new()
            .build()
            .map_err(|error| GameError::InitError(format!("{}", error)))?;
        Ok(Self {
            gilrs,
        })
    }

    pub(crate) fn gilrs_mut(&mut self) -> &mut Gilrs {
        &mut self.gilrs
    }

    pub(crate) fn clear_states(&mut self) {
        // TODO
    }

}

#[derive(Debug, Clone)]
pub struct GamepadConfig {}

impl GamepadConfig {

    pub fn new() -> Self {
        Self {}
    }

}
