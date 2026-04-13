use crate::plugins::{player::Player, util::PreviousTranslation};

use bevy::prelude::*;

pub fn startup(mut commands: Commands) {
    commands.spawn((Transform::default(), Player, PreviousTranslation::default()));
    commands.spawn((
        Transform::default().looking_to(Vec3::NEG_Z, Vec3::Y),
        Camera3d::default(),
    ));
}

// TODO: Lerp camera position between the players previous and current position using
