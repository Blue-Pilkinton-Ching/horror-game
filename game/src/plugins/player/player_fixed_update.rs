use bevy::prelude::*;

use crate::plugins::{
    input::InputActionState,
    landscape::landscape::controller::LandscapeController,
    player::{PLAYER_HEIGHT, Player},
};

pub fn fixed_update(
    input_state: Res<InputActionState>,
    landscape: Res<LandscapeController>,
    mut transform: Single<&mut Transform, With<Player>>,
) {
    transform.translation.z -= 1.0;
    transform.translation.y = landscape.sample_ground_height_at_world_position(Vec2 {
        x: transform.translation.x,
        y: transform.translation.z,
    }) + PLAYER_HEIGHT;

    if input_state.move_left {
        transform.translation.x -= 1.0;
    }

    if input_state.move_right {
        transform.translation.x += 1.0;
    }
}
