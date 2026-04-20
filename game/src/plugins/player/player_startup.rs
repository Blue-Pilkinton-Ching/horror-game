use crate::plugins::{
    landscape::{self, landscape::noise::sample_noise},
    player::Player,
    util::PreviousTranslation,
};

use bevy::prelude::*;

pub fn startup(
    mut commands: Commands,
    landscape: Res<landscape::landscape::controller::LandscapeController>,
) {
    let spawn_pos = Vec2::new(
        landscape.settings.chunk_mesh.vert_space_x
            * landscape.settings.chunk_mesh.verts_width as f32
            * 0.5,
        0.0,
    );

    let spawn_pos = Transform::default().with_translation(Vec3 {
        x: spawn_pos.x,
        y: sample_noise(spawn_pos, landscape.settings.noise.noise_fn.as_ref()),
        z: spawn_pos.y,
    });

    commands.spawn((spawn_pos, Player, PreviousTranslation::default()));
    commands.spawn((
        spawn_pos.looking_to(Vec3::NEG_Z, Vec3::Y),
        Camera3d::default(),
    ));
}
