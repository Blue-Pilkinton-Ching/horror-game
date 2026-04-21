use bevy::prelude::*;

use crate::plugins::{player::Player, shared::marker_components::MainCamera};

pub fn on_exit_game(
    mut commands: Commands,
    player: Single<Entity, With<Player>>,
    main_camera: Single<Entity, With<MainCamera>>,
) {
    commands.entity(*player).despawn();
    commands.entity(*main_camera).despawn();
}
