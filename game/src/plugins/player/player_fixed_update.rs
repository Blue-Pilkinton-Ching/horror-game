use bevy::prelude::*;

use crate::plugins::{input::InputActionState, player::Player};

pub fn fixed_update(
    input_state: Res<InputActionState>,
    mut transform: Single<&mut Transform, With<Player>>,
) {
    transform.translation.z -= 1.0;

    if input_state.move_left {
        transform.translation.x -= 1.0;
    }

    if input_state.move_right {
        transform.translation.x += 1.0;
    }
}
