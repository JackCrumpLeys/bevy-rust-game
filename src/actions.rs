use std::time::Instant;
use bevy::prelude::*;
use crate::keybind::{set_action_state, ActionState, KeyActions};
use crate::GameState;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(set_movement_actions.after(set_action_state)),
        );
    }
}

pub struct Actions {
    pub player_movement: Option<(Vec2, f32)>,
    pub player_shoot: (bool, Instant),
}

impl Default for Actions {
    fn default() -> Self {
        Actions {
            player_movement: None,
            player_shoot: (false, Instant::now()),
        }
    }
}


fn set_movement_actions(mut actions: ResMut<Actions>, action_state: Res<ActionState>) {
    let mut player_movement = Vec2::ZERO;
    let mut player_rotation = 0.0;

    let action_state = action_state.into_inner();

    if action_state[KeyActions::Up] {
        player_movement.y += 1.0;
    }

    if action_state[KeyActions::Down] {
        player_movement.y -= 1.0;
    }

    if action_state[KeyActions::Right] {
        player_movement.x += 1.0;
    }

    if action_state[KeyActions::Left] {
        player_movement.x -= 1.0;
    }

    if action_state[KeyActions::RotateLeft] {
        player_rotation -= 0.1;
    }

    if action_state[KeyActions::RotateRight] {
        player_rotation += 0.1;
        println!("rotate right");
    }

    if actions.player_shoot.1.elapsed().as_secs_f32() >= 0.1 && action_state[KeyActions::Shoot] {
        actions.player_shoot.0 = true;
        actions.player_shoot.1 = Instant::now();
    } else {
        actions.player_shoot.0 = false;
    }

    if player_movement != Vec2::ZERO || player_rotation != 0.0 {
        actions.player_movement = Some((player_movement, player_rotation));
    } else {
        actions.player_movement = None
    }

    //     if GameControl::Up.just_released(&keyboard_input)
    //         || GameControl::Down.just_released(&keyboard_input)
    //     {
    //         if GameControl::Up.pressed(&keyboard_input) {
    //             player_movement.y = 1.;
    //         } else if GameControl::Down.pressed(&keyboard_input) {
    //             player_movement.y = -1.;
    //         } else {
    //             player_movement.y = 0.;
    //         }
    //     } else if GameControl::Up.just_pressed(&keyboard_input) {
    //         player_movement.y = 1.;
    //     } else if GameControl::Down.just_pressed(&keyboard_input) {
    //         player_movement.y = -1.;
    //     } else {
    //         player_movement.y = actions.player_movement.unwrap_or(Vec2::ZERO).y;
    //     }
    //
    //     if GameControl::Right.just_released(&keyboard_input)
    //         || GameControl::Left.just_released(&keyboard_input)
    //     {
    //         if GameControl::Right.pressed(&keyboard_input) {
    //             player_movement.x = 1.;
    //         } else if GameControl::Left.pressed(&keyboard_input) {
    //             player_movement.x = -1.;
    //         } else {
    //             player_movement.x = 0.;
    //         }
    //     } else if GameControl::Right.just_pressed(&keyboard_input) {
    //         player_movement.x = 1.;
    //     } else if GameControl::Left.just_pressed(&keyboard_input) {
    //         player_movement.x = -1.;
    //     } else {
    //         player_movement.x = actions.player_movement.unwrap_or(Vec2::ZERO).x;
    //     }
    //
    //     if player_movement != Vec2::ZERO {
    //         player_movement = player_movement.normalize();
    //         actions.player_movement = Some(player_movement);
    //     }
    // } else {
    //     actions.player_movement = None;
    // }
}
