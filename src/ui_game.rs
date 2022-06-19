use crate::bullet::{insert_bullet_at, BulletOptions};
use crate::physics::TestBall;
use crate::player::{Player, PlayerSettings};
use crate::prelude::*;
use bevy_egui::EguiContext;
use egui::{Color32, Rgba, RichText};

pub struct UiGame;

#[derive(Default)]
struct Rotation {
    /// had to do this because: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `bevy::prelude::Quat` brobs better way to handle it
    x: f32,
    y: f32,
    z: f32,
}

impl Plugin for UiGame {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(player_diagnostics)
                .with_system(ball_diagnostics)
                .with_system(player_settings),
        );
    }
}

fn ball_diagnostics(
    mut egui_context: ResMut<EguiContext>,
    mut ball_query: Query<(Entity, &mut Transform), With<TestBall>>, mut commands: Commands,
) {
    egui::Window::new("ball Diagnostics").show(egui_context.ctx_mut(), |ui| {
        if ui.button("Spawn new ball").clicked() {
            commands
                .spawn()
                .insert(RigidBody::Dynamic)
                .insert(Collider::ball(0.5))
                .insert(Restitution::coefficient(0.7))
                .insert(TestBall)
                .insert_bundle(TransformBundle::from(Transform::from_xyz(2.0, 0.0, 3.0)));
        }
        for (idx, (entity, mut ball)) in ball_query.iter_mut().enumerate() {
            ui.label(RichText::new(format!("Ball {}", idx)).size(30.0).color(Rgba::GREEN));
            ui.label(format!("Location {:.2},{:.2}", ball.translation.x, ball.translation.y));
            ui.add(egui::Slider::new(&mut ball.translation.x, -50.0..=50.0).text("Translation X"));
            ui.add(egui::Slider::new(&mut ball.translation.y, -50.0..=50.0).text("Translation Y"));

            if ui.button("Reset position").clicked() {
                ball.translation = Vec3::default();
                ball.rotation = Quat::default();
            }

            if ui.button(RichText::new("kill").color(Color32::RED)).clicked() {
                commands.entity(entity).despawn()
            }
        }
    });
}

fn player_diagnostics(
    mut egui_context: ResMut<EguiContext>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // keybindings: Res<Keybindings>
) {
    egui::Window::new("Player Diagnostics").show(egui_context.ctx_mut(), |ui| {
        for (idx, mut player_transform) in player_query.iter_mut().enumerate() {
            ui.strong(format!("Player {}", idx));
            ui.label(format!(
                "Location {:.2},{:.2}",
                player_transform.translation.x, player_transform.translation.y
            ));
            ui.add(
                egui::Slider::new(&mut player_transform.translation.x, -50.0..=50.0)
                    .text("Translation X"),
            );
            ui.add(
                egui::Slider::new(&mut player_transform.translation.y, -50.0..=50.0)
                    .text("Translation Y"),
            );
            let (rot_x, rot_y, rot_z) = player_transform.rotation.to_euler(EulerRot::XYZ);
            let mut rotation = Rotation { x: rot_x, y: rot_y, z: rot_z };

            // let mut rotation = Rotation { x:player_transform.rotation.x,y:player_transform.rotation.y, z:player_transform.rotation.z, w:player_transform.rotation.w };
            ui.add(egui::Slider::new(&mut rotation.x, -3.14..=3.14).text("rotation X"));
            ui.add(egui::Slider::new(&mut rotation.y, -3.14..=3.14).text("rotation Y"));
            ui.add(egui::Slider::new(&mut rotation.z, -3.14..=3.14).text("rotation Z"));
            player_transform.rotation =
                Quat::from_euler(EulerRot::XYZ, rotation.x, rotation.y, rotation.z);

            if ui.button("spawn bullet").clicked() {
                insert_bullet_at(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    BulletOptions {
                        pos: player_transform.translation,
                        rotation: player_transform.rotation,
                    },
                );
            }
            if ui.button("Reset position").clicked() {
                player_transform.translation = Vec3::default();
                player_transform.rotation = Quat::default();
            }
        }
    });
}

fn player_settings(mut egui_context: ResMut<EguiContext>, mut settings: ResMut<PlayerSettings>) {
    egui::Window::new("Player Settings").show(egui_context.ctx_mut(), |ui| {
        ui.label(format!("Movement"));
        ui.add(egui::Slider::new(&mut settings.speed, 0.5..=50.0).text("Speed"));
    });
}
