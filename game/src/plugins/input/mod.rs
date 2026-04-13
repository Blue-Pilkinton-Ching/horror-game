use bevy::prelude::*;

mod input_update;

pub struct InputPlugin;

#[derive(Resource, Default)]
pub struct InputActionState {
    pub move_left: bool,
    pub move_right: bool,
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, input_update::update)
            .init_resource::<InputActionState>();
    }
}
