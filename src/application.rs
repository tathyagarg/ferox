use log::{info, warn};

use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, ModifiersState, PhysicalKey},
    window::{Window, WindowId},
};

use crate::inputs::{self, Binding, actions::Action};
use crate::state;

#[derive(Default)]
pub struct App {
    pub state: Option<state::State>,

    pub keybindings: Vec<Binding>,
}

impl App {
    pub fn new() -> Self {
        let keybindings = inputs::get_keybindings().to_vec();
        Self {
            state: None,
            keybindings,
        }
    }

    fn add_keybinding(&mut self, binding: Binding) {
        self.keybindings.push(binding);
    }

    pub fn get_state(&self) -> &state::State {
        self.state.as_ref().expect("State should be initialized")
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window object
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let state = pollster::block_on(state::State::new(window.clone()));
        self.state = Some(state);

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                info!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                // Reconfigures the size of the surface. We do not re-render
                // here as this event is always followed up by redraw request.
                state.resize(size);
            }
            WindowEvent::RedrawRequested => {
                state.render();
                state.frame_counter.update();

                state.get_window().request_redraw();
            }
            WindowEvent::KeyboardInput {
                event,
                is_synthetic: false,
                ..
            } => match event.physical_key {
                PhysicalKey::Code(key_code) => {
                    info!("KeyboardInput: {:?}", key_code);

                    let action = inputs::make_key_action(key_code, &self.keybindings.clone());
                    _ = inputs::handle_action(action, self, event_loop);
                }
                PhysicalKey::Unidentified(..) => {
                    warn!("Unknown key pressed");
                }
            },
            WindowEvent::MouseInput { state, button, .. } => {
                let action = inputs::make_mouse_action(button, state);
                _ = inputs::handle_action(action, self, event_loop);
            }
            WindowEvent::CursorMoved { position, .. } => {
                info!("Cursor moved to: {:?}", position);
                self.state
                    .as_mut()
                    .unwrap()
                    .update_cursor_position(position);
            }
            _ => (),
        }
    }
}

pub fn setup_keybindings(app: &mut App) {
    app.add_keybinding(Binding {
        key: Some(KeyCode::Escape),
        button: None,
        modifiers: ModifiersState::empty(),
        action: Action::CloseWindow,
    });
}
