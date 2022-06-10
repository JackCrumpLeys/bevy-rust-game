use crate::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct Physics;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for Physics {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(RapierConfiguration{
                gravity: Vect::Z * -9.81,
                ..RapierConfiguration::default()
            })
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(setup_physics)
        );
    }
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn()
        .insert(Collider::cuboid(100.0, 100.0, 0.1))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, -0.1)));

    /* Create the bouncing ball. */
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(2.0, 0.0, 3.0)));
}