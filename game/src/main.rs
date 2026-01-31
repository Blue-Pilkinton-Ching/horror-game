use bevy::prelude::*;

use crate::plugins::*;

mod plugins;
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
        .add_plugins((
            input::InputPlugin,
            world::WorldPlugin,
            player::PlayerPlugin,
            landscape::LandscapePlugin,
        ))
        .run();
}
