use bevy::prelude::*;

use crate::plugins::landscape::landscape::{chunk::Chunk, controller::LandscapeController};

pub fn on_exit_game(
    mut commands: Commands,
    mut landscape_controller: ResMut<LandscapeController>,
    chunk_entity: Query<Entity, With<Chunk>>,
) {
    landscape_controller.chunks.clear();
    for entity in chunk_entity.iter() {
        commands.entity(entity).despawn();
    }
}
