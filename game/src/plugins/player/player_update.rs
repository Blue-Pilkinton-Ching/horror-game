use bevy::prelude::*;

use crate::plugins::{
    landscape::landscape::controller::LandscapeController, player::Player,
    shared::materials::TinyWorldMaterialExt, util::PreviousTranslation,
};

pub fn player_update(
    mut camera_transform: Single<&mut Transform, (With<Camera3d>, Without<Player>)>,
    player_transform: Single<&Transform, (With<Player>, Without<Camera3d>)>,
    fixed_time: Res<Time<Fixed>>,
    player_previous_translation: Single<&PreviousTranslation, With<Player>>,
    mut materials: ResMut<Assets<TinyWorldMaterialExt>>,
    landscape: Res<LandscapeController>,
) {
    let target_translation = player_transform.translation;
    let previous_translation = player_previous_translation.value();

    // lerp between the the position of the player last physics update, and current position
    // Based on where this rendering step is between the steps
    let new = previous_translation.lerp(target_translation, fixed_time.overstep_fraction());

    camera_transform.translation = new;

    for (_id, mat) in materials.iter_mut() {
        // Set player position for tiny world materials to camera position
        mat.extension.params.player_position_z = camera_transform.translation.z;
    }

    let ground_height =
        landscape.sample_ground_height_at_world_position(Vec2::new(target_translation.x, new.z));
    let ground_height_soon = landscape
        .sample_ground_height_at_world_position(Vec2::new(target_translation.x, new.z - 2.0));

    let forward = Vec3::new(0.0, ground_height_soon - ground_height, -2.0).normalize();
    camera_transform.look_to(forward, Dir3::Y);
}
