use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct PreviousTranslation {
    value: Vec3,
}

impl PreviousTranslation {
    pub fn value(&self) -> Vec3 {
        self.value
    }

    fn set(&mut self, translation: Vec3) {
        self.value = translation;
    }
}

fn fixed_pre_update(query: Query<(&Transform, &mut PreviousTranslation)>) {
    for (transform, mut prev) in query {
        prev.set(transform.translation);
    }
}
pub struct PreviousTranslationPlugin;

impl Plugin for PreviousTranslationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedPreUpdate, fixed_pre_update);
    }
}
