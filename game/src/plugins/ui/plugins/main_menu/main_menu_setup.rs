use bevy::prelude::*;

use crate::plugins::ui::plugins::main_menu::MainMenu;

pub fn setup(mut commands: Commands) {
    commands.spawn(main_menu_ui_bundle());
}

fn main_menu_ui_bundle() -> impl Bundle {
    (
        MainMenu,
        Node {
            width: Val::Vw(100.0),
            height: Val::Vh(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            display: Display::None,
            ..Default::default()
        },
        children![(Text::new("Main menu"))],
    )
}
