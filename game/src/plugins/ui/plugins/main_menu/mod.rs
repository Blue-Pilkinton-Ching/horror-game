use bevy::prelude::*;

use crate::state::AppState;

mod main_menu_on_enter;
mod main_menu_on_exit;
mod main_menu_setup;

pub struct MenuPlugin;

#[derive(Component)]
pub struct MainMenu;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, main_menu_setup::setup)
            .add_systems(OnEnter(AppState::MainMenu), main_menu_on_enter::on_enter)
            .add_systems(OnExit(AppState::MainMenu), main_menu_on_exit::on_exit);
    }
}
