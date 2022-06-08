use bevy::prelude::*;
use bevy_egui::EguiContext;
use crate::{GameState, SystemSet};
use crate::player::{Player, PlayerSettings};

pub struct UiGame;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for UiGame {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(player_diagnostics)
                .with_system(player_settings)
        );
    }
}

fn player_diagnostics(mut egui_context: ResMut<EguiContext>, mut player_query: Query<&mut Transform, With<Player>>) {
    egui::Window::new("Player Diagnostics").show(egui_context.ctx_mut(), |ui| {

        for (idx, mut player_transform) in player_query.iter_mut().enumerate() {
            ui.label(format!("Player {}", idx));
            ui.label(format!("Location {:.2},{:.2}", player_transform.translation.x, player_transform.translation.y));
            ui.add(egui::Slider::new(&mut player_transform.translation.x, -50.0..=50.0).text("Translation X"));
            ui.add(egui::Slider::new(&mut player_transform.translation.y, -50.0..=50.0).text("Translation Y"));
        }
    });
}

fn player_settings(mut egui_context: ResMut<EguiContext>, mut settings: ResMut<PlayerSettings>) {
    egui::Window::new("Player Settings").show(egui_context.ctx_mut(), |ui| {
        ui.label(format!("Movement"));
        ui.add(egui::Slider::new(&mut settings.speed, 0.5..=50.0).text("Speed"));
    });
}
