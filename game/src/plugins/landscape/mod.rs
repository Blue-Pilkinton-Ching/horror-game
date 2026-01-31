use bevy::prelude::*;

mod landscape_startup;

pub struct LandscapePlugin;

impl Plugin for LandscapePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, landscape_startup::startup);
    }
}
