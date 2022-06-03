use std::{sync::{Arc, RwLock}};

use winit::event::VirtualKeyCode;

use crate::model::model::Model;

use super::{controller::{KeyboundFunction, up_action, half_screen_width_ingame_point5times, no_action, half_screen_width_ingame_2times, place_debug_object_action, half_screen_width_ingame_regular, simulate_mouse_wheel_up, simulate_mouse_wheel_down}, button_constants::{W_BUTTON, D_BUTTON, S_BUTTON, A_BUTTON, MOUSE_LEFT, MOUSE_RIGHT, MOUSE_MIDDLE, SPACE_BAR, CTRL, J_BUTTON, L_BUTTON}, game_state::GameState};

pub(crate) fn load_default_keybinds() -> Vec<Option<KeyboundFunction>>{
    let mut ret = Vec::new();
    //TODO: add a config file for bound defaults, fallback to code, if none is present
    //see button_constants.rs, to figure out how the indices represent different keys

    ret.resize(11, None);
    let fn_pointer: KeyboundFunction = up_action;
    ret[W_BUTTON] = Some(fn_pointer);
    let fn_pointer: KeyboundFunction = half_screen_width_ingame_point5times;
    ret[D_BUTTON] = Some(fn_pointer);
    let fn_pointer: KeyboundFunction = no_action;
    ret[S_BUTTON] = Some(fn_pointer);
    let fn_pointer: KeyboundFunction = half_screen_width_ingame_2times;
    ret[A_BUTTON] = Some(fn_pointer);
    let fn_pointer: KeyboundFunction = place_debug_object_action;
    ret[MOUSE_LEFT] = Some(fn_pointer);
    let fn_pointer: KeyboundFunction = no_action;
    ret[MOUSE_RIGHT] = Some(fn_pointer);
    let fn_pointer: KeyboundFunction = no_action;
    ret[MOUSE_MIDDLE] = Some(fn_pointer);
    let fn_pointer: KeyboundFunction = half_screen_width_ingame_regular;
    ret[SPACE_BAR] = Some(fn_pointer);
    let fn_pointer: KeyboundFunction = simulate_mouse_wheel_up;
    ret[J_BUTTON] = Some(fn_pointer);
    let fn_pointer: KeyboundFunction = simulate_mouse_wheel_down;
    ret[L_BUTTON] = Some(fn_pointer);




    ret[CTRL] = None;
    return ret;
}


pub(crate) fn key_action_pressed(key: VirtualKeyCode, game_state: &Arc<RwLock<GameState>>, keybinds: &Vec<Option<KeyboundFunction>>, model: &Arc<Model>){
    println!("Key action pressed: {:?}", key);
    match key {
        VirtualKeyCode::Key1 => (),
        VirtualKeyCode::Key2 => (),
        VirtualKeyCode::Key3 => (),
        VirtualKeyCode::Key4 => (),
        VirtualKeyCode::Key5 => (),
        VirtualKeyCode::Key6 => (),
        VirtualKeyCode::Key7 => (),
        VirtualKeyCode::Key8 => (),
        VirtualKeyCode::Key9 => (),
        VirtualKeyCode::Key0 => (),
        VirtualKeyCode::A => if let Some(func) = keybinds[A_BUTTON] { func(game_state, model)},
        VirtualKeyCode::B => (),
        VirtualKeyCode::C => (),
        VirtualKeyCode::D => if let Some(func) = keybinds[D_BUTTON] { func(game_state, model)},
        VirtualKeyCode::E => (),
        VirtualKeyCode::F => (),
        VirtualKeyCode::G => (),
        VirtualKeyCode::H => (),
        VirtualKeyCode::I => (),
        VirtualKeyCode::J => if let Some(func) = keybinds[J_BUTTON] { func(game_state, model)},
        VirtualKeyCode::K => (),
        VirtualKeyCode::L => if let Some(func) = keybinds[L_BUTTON] { func(game_state, model)},
        VirtualKeyCode::M => (),
        VirtualKeyCode::N => (),
        VirtualKeyCode::O => (),
        VirtualKeyCode::P => (),
        VirtualKeyCode::Q => (),
        VirtualKeyCode::R => (),
        VirtualKeyCode::S => if let Some(func) = keybinds[S_BUTTON] { func(game_state, model)},
        VirtualKeyCode::T => (),
        VirtualKeyCode::U => (),
        VirtualKeyCode::V => (),
        VirtualKeyCode::W => if let Some(func) = keybinds[W_BUTTON] { func(game_state, model)},
        VirtualKeyCode::X => (),
        VirtualKeyCode::Y => (),
        VirtualKeyCode::Z => (),
        VirtualKeyCode::Escape => (),
        VirtualKeyCode::F1 => (),
        VirtualKeyCode::F2 => (),
        VirtualKeyCode::F3 => (),
        VirtualKeyCode::F4 => (),
        VirtualKeyCode::F5 => (),
        VirtualKeyCode::F6 => (),
        VirtualKeyCode::F7 => (),
        VirtualKeyCode::F8 => (),
        VirtualKeyCode::F9 => (),
        VirtualKeyCode::F10 => (),
        VirtualKeyCode::F11 => (),
        VirtualKeyCode::F12 => (),
        VirtualKeyCode::F13 => (),
        VirtualKeyCode::F14 => (),
        VirtualKeyCode::F15 => (),
        VirtualKeyCode::F16 => (),
        VirtualKeyCode::F17 => (),
        VirtualKeyCode::F18 => (),
        VirtualKeyCode::F19 => (),
        VirtualKeyCode::F20 => (),
        VirtualKeyCode::F21 => (),
        VirtualKeyCode::F22 => (),
        VirtualKeyCode::F23 => (),
        VirtualKeyCode::F24 => (),
        VirtualKeyCode::Snapshot => (),
        VirtualKeyCode::Scroll => (),
        VirtualKeyCode::Pause => (),
        VirtualKeyCode::Insert => (),
        VirtualKeyCode::Home => (),
        VirtualKeyCode::Delete => (),
        VirtualKeyCode::End => (),
        VirtualKeyCode::PageDown => (),
        VirtualKeyCode::PageUp => (),
        VirtualKeyCode::Left => (),
        VirtualKeyCode::Up => (),
        VirtualKeyCode::Right => (),
        VirtualKeyCode::Down => (),
        VirtualKeyCode::Back => (),
        VirtualKeyCode::Return => (),
        VirtualKeyCode::Space => if let Some(func) = keybinds[SPACE_BAR] { func(game_state, model)},
        VirtualKeyCode::Compose => (),
        VirtualKeyCode::Caret => (),
        VirtualKeyCode::Numlock => (),
        VirtualKeyCode::Numpad0 => (),
        VirtualKeyCode::Numpad1 => (),
        VirtualKeyCode::Numpad2 => (),
        VirtualKeyCode::Numpad3 => (),
        VirtualKeyCode::Numpad4 => (),
        VirtualKeyCode::Numpad5 => (),
        VirtualKeyCode::Numpad6 => (),
        VirtualKeyCode::Numpad7 => (),
        VirtualKeyCode::Numpad8 => (),
        VirtualKeyCode::Numpad9 => (),
        VirtualKeyCode::NumpadAdd => (),
        VirtualKeyCode::NumpadDivide => (),
        VirtualKeyCode::NumpadDecimal => (),
        VirtualKeyCode::NumpadComma => (),
        VirtualKeyCode::NumpadEnter => (),
        VirtualKeyCode::NumpadEquals => (),
        VirtualKeyCode::NumpadMultiply => (),
        VirtualKeyCode::NumpadSubtract => (),
        VirtualKeyCode::AbntC1 => (),
        VirtualKeyCode::AbntC2 => (),
        VirtualKeyCode::Apostrophe => (),
        VirtualKeyCode::Apps => (),
        VirtualKeyCode::Asterisk => (),
        VirtualKeyCode::At => (),
        VirtualKeyCode::Ax => (),
        VirtualKeyCode::Backslash => (),
        VirtualKeyCode::Calculator => (),
        VirtualKeyCode::Capital => (),
        VirtualKeyCode::Colon => (),
        VirtualKeyCode::Comma => (),
        VirtualKeyCode::Convert => (),
        VirtualKeyCode::Equals => (),
        VirtualKeyCode::Grave => (),
        VirtualKeyCode::Kana => (),
        VirtualKeyCode::Kanji => (),
        VirtualKeyCode::LAlt => (),
        VirtualKeyCode::LBracket => (),
        VirtualKeyCode::LControl => (),
        VirtualKeyCode::LShift => (),
        VirtualKeyCode::LWin => (),
        VirtualKeyCode::Mail => (),
        VirtualKeyCode::MediaSelect => (),
        VirtualKeyCode::MediaStop => (),
        VirtualKeyCode::Minus => (),
        VirtualKeyCode::Mute => (),
        VirtualKeyCode::MyComputer => (),
        VirtualKeyCode::NavigateForward => (),
        VirtualKeyCode::NavigateBackward => (),
        VirtualKeyCode::NextTrack => (),
        VirtualKeyCode::NoConvert => (),
        VirtualKeyCode::OEM102 => (),
        VirtualKeyCode::Period => (),
        VirtualKeyCode::PlayPause => (),
        VirtualKeyCode::Plus => (),
        VirtualKeyCode::Power => (),
        VirtualKeyCode::PrevTrack => (),
        VirtualKeyCode::RAlt => (),
        VirtualKeyCode::RBracket => (),
        VirtualKeyCode::RControl => (),
        VirtualKeyCode::RShift => (),
        VirtualKeyCode::RWin => (),
        VirtualKeyCode::Semicolon => (),
        VirtualKeyCode::Slash => (),
        VirtualKeyCode::Sleep => (),
        VirtualKeyCode::Stop => (),
        VirtualKeyCode::Sysrq => (),
        VirtualKeyCode::Tab => (),
        VirtualKeyCode::Underline => (),
        VirtualKeyCode::Unlabeled => (),
        VirtualKeyCode::VolumeDown => (),
        VirtualKeyCode::VolumeUp => (),
        VirtualKeyCode::Wake => (),
        VirtualKeyCode::WebBack => (),
        VirtualKeyCode::WebFavorites => (),
        VirtualKeyCode::WebForward => (),
        VirtualKeyCode::WebHome => (),
        VirtualKeyCode::WebRefresh => (),
        VirtualKeyCode::WebSearch => (),
        VirtualKeyCode::WebStop => (),
        VirtualKeyCode::Yen => (),
        VirtualKeyCode::Copy => (),
        VirtualKeyCode::Paste => (),
        VirtualKeyCode::Cut => (),
    }
}

pub(crate) fn key_action_released(key: VirtualKeyCode, game_state: &Arc<RwLock<GameState>>, keybinds: &Vec<Option<KeyboundFunction>>, model: &Arc<Model>){
    println!("Key_action_released in controller/button_mapping.rs not yet implemented");
}