use bevy::prelude::*;

mod player_fixed_update;
mod player_startup;
mod player_update;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_startup::startup)
            .add_systems(FixedUpdate, player_fixed_update::fixed_update)
            .add_systems(Update, player_update::player_update);
    }
}

#[derive(Component)]
pub struct Player;
