use bevy::prelude::*;

pub fn update(keys: Res<ButtonInput<KeyCode>>, mut input_state: ResMut<super::InputActionState>) {
    // TODO: Handle other forms of input (gamepads?, mouse?, mobile touch? )
    const LEFT_KEYS: [KeyCode; 3] = [KeyCode::KeyA, KeyCode::ArrowLeft, KeyCode::KeyJ];

    if LEFT_KEYS.iter().any(|key| keys.pressed(*key)) {
        input_state.move_left = true;
    } else {
        input_state.move_left = false;
    }

    const RIGHT_KEYS: [KeyCode; 3] = [KeyCode::KeyD, KeyCode::ArrowRight, KeyCode::KeyL];

    if RIGHT_KEYS.iter().any(|key| keys.pressed(*key)) {
        input_state.move_right = true;
    } else {
        input_state.move_right = false;
    }
}
