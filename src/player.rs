use crate::actions::Actions;
use crate::prelude::*;
use bevy_rapier3d::prelude::RigidBody;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

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
        .insert(Player);
}

fn move_player(
    time: Res<Time>, actions: Res<Actions>, settings: Res<PlayerSettings>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = settings.speed;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.0,
    );
    for mut player_transform in player_query.iter_mut() {
        player_transform.translation += movement;
    }
}
