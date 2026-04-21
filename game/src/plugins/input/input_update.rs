use bevy::prelude::*;

const LEFT_KEYS: [KeyCode; 3] = [KeyCode::KeyA, KeyCode::ArrowLeft, KeyCode::KeyJ];
const RIGHT_KEYS: [KeyCode; 3] = [KeyCode::KeyD, KeyCode::ArrowRight, KeyCode::KeyL];

const KILL_KEYS: [KeyCode; 1] = [KeyCode::KeyK];

pub fn update(keys: Res<ButtonInput<KeyCode>>, mut input_state: ResMut<super::InputActionState>) {
    // TODO: Handle other forms of input (gamepads?, mouse?, mobile touch? )

    if LEFT_KEYS.iter().any(|key| keys.pressed(*key)) {
        input_state.move_left = true;
    } else {
        input_state.move_left = false;
    }

    if RIGHT_KEYS.iter().any(|key| keys.pressed(*key)) {
        input_state.move_right = true;
    } else {
        input_state.move_right = false;
    }

    if KILL_KEYS.iter().any(|key| keys.pressed(*key)) {
        input_state.kill = true;
    }
}
