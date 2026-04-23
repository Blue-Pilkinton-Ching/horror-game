use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod main_menu;

pub struct UIPlugins;

impl PluginGroup for UIPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(main_menu::MenuPlugin)
    }
}
