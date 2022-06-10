use crate::loading::FontAssets;
use crate::prelude::*;
use bevy_egui::EguiContext;
use egui::{Rgba, RichText};

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Menu).with_system(ui_example));
    }
}

fn ui_example(mut egui_context: ResMut<EguiContext>, mut state: ResMut<State<GameState>>) {
    egui::Window::new("Menu").show(egui_context.ctx_mut(), |ui| {

        if ui.button(RichText::new("play!").color(Rgba::GREEN).size(100.0)).clicked(){
            state.set(GameState::Playing).unwrap();
        }
    });
}
