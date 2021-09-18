use crate::turnState::PollInput;
use crate::components::InputComponent;
use bracket_terminal::prelude::VirtualKeyCode;
use bracket_terminal::prelude::*;

// pub fn get_input_from_keyboard_and_moues() -> Option<InputComponent>{
//     let input = INPUT.lock();
//     let mut result = InputComponent::new();
//     if input.is_key_pressed(VirtualKeyCode::Left) |
//         input.is_key_pressed(VirtualKeyCode::Numpad4) |
//         input.is_key_pressed(VirtualKeyCode::A) {
//         result.left = true;
//         return Some(result)
//     }else if input.is_key_pressed(VirtualKeyCode::Right) |
//         input.is_key_pressed(VirtualKeyCode::Numpad6) |
//         input.is_key_pressed(VirtualKeyCode::D) {
//         result.right = true;
//         return Some(result)
//     } else if input.is_key_pressed(VirtualKeyCode::Up) |
//         input.is_key_pressed(VirtualKeyCode::Numpad8) |
//         input.is_key_pressed(VirtualKeyCode::W) {
//         result.up = true;
//         return Some(result)
//     } else if input.is_key_pressed(VirtualKeyCode::Down) |
//         input.is_key_pressed(VirtualKeyCode::Numpad2) |
//         input.is_key_pressed(VirtualKeyCode::S) {
//         result.down = true;
//         return Some(result)
//     } else if  input.is_key_pressed(VirtualKeyCode::Numpad9) |
//         input.is_key_pressed(VirtualKeyCode::E) {
//         result.right_up = true;
//         return Some(result)
//     } else if input.is_key_pressed(VirtualKeyCode::Numpad7) |
//         input.is_key_pressed(VirtualKeyCode::Q) {
//         result.left_up = true;
//         return Some(result)
//     }else if input.is_key_pressed(VirtualKeyCode::Numpad3) |
//         input.is_key_pressed(VirtualKeyCode::C) {
//         result.right_down = true;
//         return Some(result)
//     }else if input.is_key_pressed(VirtualKeyCode::Numpad1) |
//         input.is_key_pressed(VirtualKeyCode::Z) {
//         result.left_down = true;
//         return Some(result)
//     }
//
//     None
// }

pub fn get_input_for_AI(ctx:&BTerm) ->  Option<InputComponent>{
    Some(InputComponent::new())
}

pub fn get_input_from_keyboard_and_moues(ctx:&BTerm) -> Option<InputComponent>{
    // Player movement
    let mut result = InputComponent::new();
    match ctx.key {
        None => { return None } // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4 |
            VirtualKeyCode::A => {result.left = true; return Some(result)},

            VirtualKeyCode::Right |
            VirtualKeyCode::Numpad6 |
            VirtualKeyCode::D => {result.right = true; return Some(result)},

            VirtualKeyCode::Up |
            VirtualKeyCode::Numpad8 |
            VirtualKeyCode::W => {result.up = true; return Some(result)},

            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2 |
            VirtualKeyCode::S => {result.down = true; return Some(result)},

            VirtualKeyCode::Numpad9 |
            VirtualKeyCode::E  => {result.right_up = true; return Some(result)},

            VirtualKeyCode::Numpad7 |
            VirtualKeyCode::Q => {result.left_up = true; return Some(result)},

            VirtualKeyCode::Numpad3 |
            VirtualKeyCode::C => {result.right_down = true; return Some(result)},

            VirtualKeyCode::Numpad1 |
            VirtualKeyCode::Z => {result.left_down = true; return Some(result)},

            _ => { }
        },
    }
    None
}