use bevy::prelude::*;

use crate::plugins::{
    player::Player, shared::materials::TinyWorldMaterialExt, util::PreviousTranslation,
};

pub fn player_update(
    mut camera_transform: Single<&mut Transform, (With<Camera3d>, Without<Player>)>,
    player_transform: Single<&Transform, (With<Player>, Without<Camera3d>)>,
    fixed_time: Res<Time<Fixed>>,
    player_previous_translation: Single<&PreviousTranslation, With<Player>>,
    mut materials: ResMut<Assets<TinyWorldMaterialExt>>,
) {
    let target = player_transform.translation;
    let previous = player_previous_translation.value();

    // lerp between the the position of the player last physics update, and current position
    // Based on where this rendering step is between the steps
    let new = previous.lerp(target, fixed_time.overstep_fraction());

    camera_transform.translation = new;

    for (_id, mat) in materials.iter_mut() {
        // Set player position for tiny world materials to camera position
        mat.extension.params.player_position_z = camera_transform.translation.z;
    }
}
