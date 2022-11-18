use crate::GameState;
use bevy::prelude::*;
use std::collections::HashMap;
use std::ops::Index;

pub struct KeybindingsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for KeybindingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActionState>()
            .init_resource::<Keybindings>()
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(set_action_state));
    }
}

/// enum for keybindable actions
#[derive(Eq, Hash, PartialEq, Debug)]
pub enum KeyActions {
    // game control
    Up,
    Down,
    Left,
    Right,
    RotateLeft,
    RotateRight,
    Shoot,
}

/// list of player settable keybindings
pub struct Keybindings {
    map: HashMap<KeyActions, KeyCode>,
}

impl Default for Keybindings {
    fn default() -> Self {
        let mut map: HashMap<KeyActions, KeyCode> = HashMap::new();

        map.insert(KeyActions::Up, KeyCode::W);
        map.insert(KeyActions::Down, KeyCode::S);
        map.insert(KeyActions::Left, KeyCode::A);
        map.insert(KeyActions::Right, KeyCode::D);
        map.insert(KeyActions::RotateLeft, KeyCode::Q);
        map.insert(KeyActions::RotateRight, KeyCode::E);
        map.insert(KeyActions::Shoot, KeyCode::Space);

        Keybindings { map }
    }
}

impl Index<KeyActions> for Keybindings {
    type Output = KeyCode;

    fn index(&self, index: KeyActions) -> &Self::Output {
        &self.map[&index]
    }
}

/// a bool map for the current state of keybindable actions
#[derive(Debug)]
pub struct ActionState {
    map: HashMap<KeyActions, bool>,
}

impl Default for ActionState {
    fn default() -> Self {
        let mut map: HashMap<KeyActions, bool> = HashMap::new();

        map.insert(KeyActions::Up, false);
        map.insert(KeyActions::Down, false);
        map.insert(KeyActions::Left, false);
        map.insert(KeyActions::Right, false);
        map.insert(KeyActions::RotateLeft, false);
        map.insert(KeyActions::RotateRight, false);
        map.insert(KeyActions::Shoot, false);

        ActionState { map }
    }
}

impl Index<KeyActions> for ActionState {
    type Output = bool;

    fn index(&self, index: KeyActions) -> &Self::Output {
        &self.map[&index]
    }
}

/// system that sets the action state based on the current keyboard input
pub fn set_action_state(
    keyboard_input: Res<Input<KeyCode>>, mut action_state: ResMut<ActionState>,
    keybindings: Res<Keybindings>,
) {
    for (k, v) in action_state.map.iter_mut() {
        *v = keyboard_input.pressed(*keybindings.map.get(k).unwrap());
    }
}
