use bevy::prelude::*;

mod player_fixed_update;
mod player_startup;
mod player_update;
pub struct PlayerPlugin;

// Meters
pub const PLAYER_HEIGHT: f32 = 1.8;
// Meters per second
pub const PLAYER_RUN_SPEED: f32 = 80.0;
pub const PLAYER_STRAFE_SPEED: f32 = 30.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_startup::startup)
            .add_systems(FixedUpdate, player_fixed_update::fixed_update)
            .add_systems(Update, player_update::player_update);
    }
}

#[derive(Component, Debug)]
pub struct Player;
