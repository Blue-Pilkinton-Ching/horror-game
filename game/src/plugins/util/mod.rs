use bevy::app::*;

mod plugins;
pub use plugins::previous_translation::PreviousTranslation;
pub struct UtilPlugins;

impl PluginGroup for UtilPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(plugins::previous_translation::PreviousTranslationPlugin)
    }
}
