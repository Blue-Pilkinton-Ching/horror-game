use bevy::prelude::*;

mod world_startup;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, world_startup::startup);
    }
}
