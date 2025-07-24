pub mod actions;
pub mod event_handler;

use log::info;
use std::collections::HashMap;
use std::sync::Mutex;
use winit::{
    event::{ElementState, MouseButton},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, ModifiersState},
};

use crate::application::App;
use actions::Action;

#[derive(Clone)]
pub struct Binding {
    pub key: Option<KeyCode>,
    pub button: Option<MouseButton>,
    pub modifiers: ModifiersState,
    pub action: Action,
}

const KEY_BINDINGS: &[Binding] = &[
    Binding {
        key: None,
        button: Some(MouseButton::Left),
        modifiers: ModifiersState::empty(),
        action: Action::ButtonPress(MouseButton::Left, ElementState::Pressed),
    },
    Binding {
        key: None,
        button: Some(MouseButton::Right),
        modifiers: ModifiersState::empty(),
        action: Action::ButtonPress(MouseButton::Right, ElementState::Pressed),
    },
];

pub fn get_keybindings() -> &'static [Binding] {
    KEY_BINDINGS
}

pub fn make_key_action(event: KeyCode, bindings: &[Binding]) -> Action {
    for binding in bindings.iter() {
        if let Some(key) = binding.key {
            if key == event {
                return binding.action.clone();
            }
        }
    }

    Action::Keyboard(event, ElementState::Pressed)
}

pub fn make_mouse_action(button: MouseButton, state: ElementState) -> Action {
    Action::ButtonPress(button, state)
}

pub fn handle_action(
    action: Action,
    app: &mut App,
    event_loop: &ActiveEventLoop,
) -> Result<(), String> {
    match action {
        Action::CloseWindow => {
            info!("Closing window");

            event_loop.exit();
        }
        Action::ButtonPress(button, state) => {
            info!(
                "Button {:?}: {:?} at {:?}",
                state,
                button,
                app.get_state().get_cursor_position()
            );
        }
        Action::Keyboard(key, state) => event_handler::event_handlers
            .get_or_init(|| Mutex::new(HashMap::new()))
            .lock()
            .unwrap()
            .get(&key)
            .map_or_else(
                || {
                    info!("No handler for key: {:?}", key);
                    Err(format!("No handler for key: {:?}", key))
                },
                |handler| {
                    info!("Handling key: {:?} with state: {:?}", key, state);
                    handler(app, event_loop);
                    Ok(())
                },
            )?,
    };

    info!("Action handled: {:?}", action);
    Ok(())
}
