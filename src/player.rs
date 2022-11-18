use crate::actions::Actions;
use crate::prelude::*;
use bevy_rapier3d::prelude::RigidBody;

pub struct PlayerPlugin;

#[derive(Component, Default)]
pub struct Player {
    pub score: usize,
}

pub struct PlayerSettings {
    pub speed: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        PlayerSettings { speed: 2.0 }
    }
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_player))
            .insert_resource(PlayerSettings::default());
    }
}

fn spawn_player(
    mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(1.5, 0.5, 0.0),
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(CameraFocus::default())
        .insert(Player::default());
}

// fn set_movement_actions(mut actions: ResMut<Actions>, action_state: Res<ActionState>) {
//     let mut player_movement = Vec2::ZERO;
//     let mut player_rotation = 0;
//
//     let action_state = action_state.into_inner();
//
//     if action_state[KeyActions::Up] {
//         player_movement.y += 1.0;
//     }
//
//     if action_state[KeyActions::Down] {
//         player_movement.y -= 1.0;
//     }
//
//     if action_state[KeyActions::Right] {
//         player_movement.x += 1.0;
//     }
//
//     if action_state[KeyActions::Left] {
//         player_movement.x -= 1.0;
//     }
//
//     if action_state[KeyActions::RotateLeft] {
//         player_rotation -= 1;
//     }
//
//     if action_state[KeyActions::RotateRight] {
//         player_rotation += 1;
//     }
//
//     if actions.player_shoot.1.elapsed().as_secs_f32() >= 0.1 && action_state[KeyActions::Shoot] {
//         actions.player_shoot.0 = true;
//         actions.player_shoot.1 = Instant::now();
//     } else {
//         actions.player_shoot.0 = false;
//     }
//
//     if player_movement != Vec2::ZERO {
//         actions.player_movement = Some((player_movement, player_rotation));
//     } else {
//         actions.player_movement = None
//     }
fn move_player(
    time: Res<Time>, actions: Res<Actions>, settings: Res<PlayerSettings>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = settings.speed;
    let movement = Vec3::new(
        actions.player_movement.unwrap().0.x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().0.y * speed * time.delta_seconds(),
        0.0,
    );
    for mut player_transform in player_query.iter_mut() {
        player_transform.translation += movement;
        // rotation
        player_transform.rotation *= Quat::from_rotation_z(actions.player_movement.unwrap().1);
    }
}
