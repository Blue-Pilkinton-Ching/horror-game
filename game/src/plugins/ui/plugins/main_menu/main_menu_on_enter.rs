use bevy::prelude::*;

use crate::plugins::{shared::marker_components::MainCamera, ui::plugins::main_menu::MainMenu};

pub fn on_enter(mut commands: Commands, mut main_menu_node: Single<&mut Node, With<MainMenu>>) {
    commands.spawn((Camera3d::default(), MainCamera));

    main_menu_node.display = Display::Flex;
}
