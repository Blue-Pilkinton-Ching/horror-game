use bevy::prelude::*;

use crate::plugins::{
    input::InputActionState,
    landscape::landscape::controller::LandscapeController,
    player::{PLAYER_HEIGHT, PLAYER_RUN_SPEED, PLAYER_STRAFE_SPEED, Player},
};

pub fn fixed_update(
    input_state: Res<InputActionState>,
    landscape: Res<LandscapeController>,
    mut transform: Single<&mut Transform, With<Player>>,
) {
    // 60 ticks per second
    transform.translation.z -= PLAYER_RUN_SPEED / 60.0;
    transform.translation.y = landscape.sample_ground_height_at_world_position(Vec2 {
        x: transform.translation.x,
        y: transform.translation.z,
    }) + PLAYER_HEIGHT;

    if input_state.move_left {
        transform.translation.x -= PLAYER_STRAFE_SPEED / 60.0;
    }

    if input_state.move_right {
        transform.translation.x += PLAYER_STRAFE_SPEED / 60.0;
    }
}
