use log::info;
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode};

use crate::App;

#[derive(Clone)]
pub enum Action {
    CloseWindow,
}

struct Modifiers {
    shift: bool,
    ctrl: bool,
    alt: bool,
}

struct Binding {
    key: KeyCode,
    modifiers: Modifiers,
    action: Action,
}

const KEY_BINDINGS: &[Binding] = &[Binding {
    key: KeyCode::Escape,
    modifiers: Modifiers {
        shift: false,
        ctrl: false,
        alt: false,
    },
    action: Action::CloseWindow,
}];

pub fn make_key_action(event: KeyCode) -> Option<Action> {
    for binding in KEY_BINDINGS {
        if event == binding.key {
            return Some(binding.action.clone());
        }
    }
    None
}

pub fn handle_action(
    action: Action,
    _app: &mut App,
    event_loop: &ActiveEventLoop,
) -> Result<(), String> {
    match action {
        Action::CloseWindow => {
            info!("Closing window");

            event_loop.exit();

            Ok(())
        }
    }
}
