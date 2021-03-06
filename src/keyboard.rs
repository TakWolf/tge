mod code;
mod modifier;

pub use code::KeyCode;
pub use modifier::ModifiersState;

use crate::error::GameResult;
use crate::event::{KeyState, KeyAction};
use std::collections::HashMap;

pub struct Keyboard {
    key_states: HashMap<KeyCode, KeyState>,
    modifiers_state: ModifiersState,
}

impl Keyboard {
    pub(crate) fn new(_: KeyboardConfig) -> GameResult<Self> {
        Ok(Self {
            key_states: HashMap::new(),
            modifiers_state: ModifiersState {
                shift: false,
                ctrl: false,
                alt: false,
                logo: false,
            },
        })
    }

    pub(crate) fn handle_input_event(&mut self, key: KeyCode, action: KeyAction) -> bool {
        let new_state = action.into();
        if let Some(state) = self.key_states.get_mut(&key) {
            if state == &new_state || (state == &KeyState::Hold && new_state == KeyState::Down) {
                true
            } else {
                *state = new_state;
                false
            }
        } else {
            self.key_states.insert(key, new_state);
            false
        }
    }

    pub(crate) fn handle_modifiers_state_change(&mut self, state: ModifiersState) {
        self.modifiers_state = state;
    }

    pub(crate) fn clear_states(&mut self) {
        self.key_states.retain(|_, state| match state {
            KeyState::Down | KeyState::Hold => {
                *state = KeyState::Hold;
                true
            }
            KeyState::Up | KeyState::Idle => false,
        });
    }

    pub fn is_key_down(&self, key: KeyCode) -> bool {
        match self.key_states.get(&key).unwrap_or(&KeyState::Idle) {
            KeyState::Down => true,
            _ => false,
        }
    }

    pub fn is_key_hold(&self, key: KeyCode) -> bool {
        match self.key_states.get(&key).unwrap_or(&KeyState::Idle) {
            KeyState::Down | KeyState::Hold => true,
            _ => false,
        }
    }

    pub fn is_key_up(&self, key: KeyCode) -> bool {
        match self.key_states.get(&key).unwrap_or(&KeyState::Idle) {
            KeyState::Up => true,
            _ => false,
        }
    }

    pub fn modifiers_state(&self) -> ModifiersState {
        self.modifiers_state
    }
}

#[derive(Debug, Clone)]
pub struct KeyboardConfig {}

impl KeyboardConfig {
    pub fn new() -> Self {
        Self {}
    }
}
