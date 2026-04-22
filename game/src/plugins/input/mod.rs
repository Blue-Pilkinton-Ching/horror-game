use bevy::prelude::*;

use crate::state::AppState;

mod input_update;

pub struct InputPlugin;

#[derive(Resource, Default)]
pub struct InputActionState {
    pub move_left: bool,
    pub move_right: bool,
    pub kill: bool,
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            input_update::game_update.run_if(in_state(AppState::InGame)),
        )
        .init_resource::<InputActionState>();
    }
}
