use bevy::prelude::*;

use crate::plugins::{player::Player, util::PreviousTranslation};

pub fn player_update(
    mut camera_transform: Single<&mut Transform, (With<Camera3d>, Without<Player>)>,
    player_transform: Single<&Transform, (With<Player>, Without<Camera3d>)>,
    fixed_time: Res<Time<Fixed>>,
    player_previous_translation: Single<&PreviousTranslation, With<Player>>,
) {
    let target = player_transform.translation;
    let previous = player_previous_translation.value();

    // lerp between the the position of the player last physics update, and current position
    // Based on where this rendering step is between the steps
    let new = previous.lerp(target, fixed_time.overstep_fraction());

    camera_transform.translation = new;
}
