use crate::GameState;
use bevy::pbr::PbrBundle;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::actions::Actions;
use crate::physics::TestBall;
use crate::player::Player;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Playing).with_system(bullet_update).with_system(fire_bullet).with_system(bullet_collide));
    }
}

pub struct BulletOptions {
    pub pos: Vec3,
    pub rotation: Quat,
}

#[derive(Component)]
pub struct Bullet;

pub fn insert_bullet_at(
    commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut  ResMut<Assets<StandardMaterial>>, options: BulletOptions,
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

pub fn fire_bullet(
    mut player_query: Query<&mut Transform, With<Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    actions: Res<Actions>
) {
    if actions.player_shoot.0 {
        for player_transform in player_query.iter_mut() {
            insert_bullet_at(&mut commands, &mut meshes, &mut materials, BulletOptions {
                pos: player_transform.translation,
                rotation: player_transform.rotation,
            });
        }
    }
}
// fn ball_diagnostics(
//     mut egui_context: ResMut<EguiContext>,
//     mut ball_query: Query<(Entity, &mut Transform), With<TestBall>>, mut commands: Commands,
// ) {
//     egui::Window::new("ball Diagnostics").show(egui_context.ctx_mut(), |ui| {
//         if ui.button("Spawn new ball").clicked() {
//             commands
//                 .spawn()
//                 .insert(RigidBody::Dynamic)
//                 .insert(Collider::ball(0.5))
//                 .insert(Restitution::coefficient(0.7))
//                 .insert(TestBall)
//                 .insert_bundle(TransformBundle::from(Transform::from_xyz(2.0, 0.0, 3.0)));
//         }
//         for (idx, (entity, mut ball)) in ball_query.iter_mut().enumerate() {
//             ui.label(RichText::new(format!("Ball {}", idx)).size(30.0).color(Rgba::GREEN));
//             ui.label(format!("Location {:.2},{:.2}", ball.translation.x, ball.translation.y));
//             ui.add(egui::Slider::new(&mut ball.translation.x, -50.0..=50.0).text("Translation X"));
//             ui.add(egui::Slider::new(&mut ball.translation.y, -50.0..=50.0).text("Translation Y"));
//
//             if ui.button("Reset position").clicked() {
//                 ball.translation = Vec3::default();
//                 ball.rotation = Quat::default();
//             }
//
//             if ui.button(RichText::new("kill").color(Color32::RED)).clicked() {
//                 commands.entity(entity).despawn()
//             }
//         }
//     });
// }


fn bullet_update(mut bullet_query: Query<&mut Transform, With<Bullet>>, time: Res<Time>) {
    for mut bullet_transform in bullet_query.iter_mut() {
        let velocity = (bullet_transform.local_x() * time.delta_seconds()).normalize();

        bullet_transform.translation += velocity;
    }
}

fn bullet_collide(ball_query: Query<(Entity, &Transform), With<TestBall>>, bullet_query: Query<(Entity, &Transform), With<Bullet>>, mut commands: Commands,) {
    // if a bullet hits a ball then the ball should be destroyed
    for (entity, ball) in ball_query.iter() {
        for (bullet_entity, bullet) in bullet_query.iter() {
            let ball: &Transform = ball;
            let bullet: &Transform = bullet;

            println!("{}",bullet.scale.x.powi(2));
            if ball.translation.distance_squared(bullet.translation) < bullet.scale.x.powi(2) {
                commands.entity(entity).despawn();
                commands.entity(bullet_entity).despawn();
            }
        }
    }
}



