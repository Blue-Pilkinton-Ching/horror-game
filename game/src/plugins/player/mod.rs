use bevy::prelude::*;

use crate::state::AppState;

mod player_fixed_update;
mod player_on_enter;
mod player_on_exit;
mod player_update;
pub struct PlayerPlugin;

// Meters
pub const PLAYER_HEIGHT: f32 = 1.8;
// Meters per second
pub const PLAYER_RUN_SPEED: f32 = 80.0;
pub const PLAYER_STRAFE_SPEED: f32 = 30.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), player_on_enter::on_enter_game)
            .add_systems(OnExit(AppState::InGame), player_on_exit::on_exit_game)
            .add_systems(
                FixedUpdate,
                (player_fixed_update::fixed_update).run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                Update,
                player_update::player_update.run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Component, Debug)]
pub struct Player;

#[derive(Event)]
struct PlayerDeath(Player);
