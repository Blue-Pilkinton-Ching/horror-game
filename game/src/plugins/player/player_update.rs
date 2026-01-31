use bevy::prelude::*;

use crate::plugins::{input::InputActionState, player::Player};

pub fn update(
    mut commands: Commands,
    input_state: Res<InputActionState>,
    mut player: Single<(&Player, &mut Transform)>,
) {
    if input_state.move_left {
        player.1.translation.x -= 0.1;
    }

    if input_state.move_right {
        player.1.translation.x -= 0.1;
    }
}
