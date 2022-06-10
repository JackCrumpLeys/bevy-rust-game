use crate::loading::TextureAssets;
use crate::player::Player;
use bevy::pbr::PbrBundle;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::GameState;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(bullet_update));
    }
}

pub struct BulletOptions {
    pub pos: Vec3,
    pub rotation: Quat,
}

#[derive(Component)]
pub struct Bullet;

pub fn insert_bullet_at(
    mut commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
    options: BulletOptions,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.25 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::default()
                .with_translation(options.pos)
                .with_rotation(options.rotation),
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(0.175, 0.175, 0.175))

        .insert(Bullet);
}
fn bullet_update(mut bullet_query: Query<&mut Transform, With<Bullet>>, time: Res<Time>) {
    for mut bullet_transform in bullet_query.iter_mut() {
        let velocity = (bullet_transform.local_x() * time.delta_seconds()).normalize();

        bullet_transform.translation += velocity;
    }
}
