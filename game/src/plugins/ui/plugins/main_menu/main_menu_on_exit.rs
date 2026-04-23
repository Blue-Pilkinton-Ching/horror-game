use bevy::prelude::*;

use crate::plugins::{shared::marker_components::MainCamera, ui::plugins::main_menu::MainMenu};

pub fn on_exit(
    mut commands: Commands,
    mut main_menu_node: Single<&mut Node, With<MainMenu>>,
    camera_entity: Single<Entity, With<MainCamera>>,
) {
    commands.entity(camera_entity.entity()).despawn();

    main_menu_node.display = Display::None;
}
