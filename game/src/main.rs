use bevy::prelude::*;

use crate::{
    plugins::{shared::materials::TinyWorldMaterialExt, *},
    state::AppState,
};

mod plugins;
mod state;
fn main() {
    let mut window_plugin = WindowPlugin::default();
    window_plugin.primary_window = Some(Window {
        title: "Game".to_string(),
        canvas: Some("#game_canvas".to_string()),
        ..Default::default()
    });
    let default_plugins = DefaultPlugins.set(window_plugin);
    App::new()
        .add_plugins(default_plugins)
        .insert_state(AppState::InGame)
        .add_plugins((
            UtilPlugins,
            MaterialPlugin::<TinyWorldMaterialExt>::default(),
            input::InputPlugin,
            player::PlayerPlugin,
            landscape::LandscapePlugin,
            UIPlugins,
        ))
        .run();
}
