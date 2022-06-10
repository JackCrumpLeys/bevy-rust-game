use crate::player::{Player, PlayerSettings};
use crate::{GameState, SystemSet};
use bevy::prelude::*;
use bevy_egui::EguiContext;
use crate::bullet::{BulletOptions, insert_bullet_at};

pub struct UiGame;

#[derive(Default)]
struct Rotation { /// had to do this because: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `bevy::prelude::Quat` brobs better way to handle it
    x:f32,y:f32,z:f32
}

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for UiGame {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(player_diagnostics)
                .with_system(player_settings),
        );
    }
}

fn player_diagnostics(
    mut egui_context: ResMut<EguiContext>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    egui::Window::new("Player Diagnostics").show(egui_context.ctx_mut(), |ui| {
        for (idx, mut player_transform) in player_query.iter_mut().enumerate() {
            ui.label(format!("Player {}", idx));
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
            let mut rotation = Rotation{x: rot_x, y: rot_y, z: rot_z};

            // let mut rotation = Rotation { x:player_transform.rotation.x,y:player_transform.rotation.y, z:player_transform.rotation.z, w:player_transform.rotation.w };
            ui.add(
                egui::Slider::new(&mut rotation.x, -3.14..=3.14)
                    .text("rotation X"),
            );
            ui.add(
                egui::Slider::new(&mut rotation.y, -3.14..=3.14)
                    .text("rotation Y"),
            );
            ui.add(
                egui::Slider::new(&mut rotation.z, -3.14..=3.14)
                    .text("rotation Z"),
            );
            player_transform.rotation = Quat::from_euler(EulerRot::XYZ, rotation.x, rotation.y, rotation.z);

            if ui.button("spawn bullet").clicked() {
                insert_bullet_at(&mut commands, &mut meshes, &mut materials, BulletOptions {
                    pos: player_transform.translation,
                    rotation: player_transform.rotation
                });
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
