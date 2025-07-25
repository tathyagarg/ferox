use log::{info, warn};
#[cfg(target_arch = "wasm32")]
use winit::event_loop::{EventLoop, EventLoopProxy};

use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, ModifiersState, PhysicalKey},
    window::{Window, WindowId},
};

use crate::state;
use crate::{
    inputs::{self, Binding, actions::Action},
    state::State,
};

#[derive(Default)]
pub struct App {
    #[cfg(target_arch = "wasm32")]
    proxy: Option<EventLoopProxy<State>>,
    state: Option<state::State>,

    pub keybindings: Vec<Binding>,
}

impl App {
    pub fn new(#[cfg(target_arch = "wasm32")] event_loop: &EventLoop<State>) -> Self {
        let keybindings = inputs::get_keybindings().to_vec();

        #[cfg(target_arch = "wasm32")]
        let proxy = Some(event_loop.create_proxy());

        Self {
            #[cfg(target_arch = "wasm32")]
            proxy,
            state: None,
            keybindings,
        }
    }

    // pub fn from_window(window: Arc<Window>) -> Self {
    //     let state = pollster::block_on(state::State::new(window)).unwrap();
    //     let keybindings = inputs::get_keybindings().to_vec();
    //     Self {
    //         state: Some(state),
    //         keybindings,
    //     }
    // }

    fn add_keybinding(&mut self, binding: Binding) {
        self.keybindings.push(binding);
    }

    pub fn get_state(&self) -> &state::State {
        self.state.as_ref().expect("State should be initialized")
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::{JsCast, UnwrapThrowExt};
            use winit::platform::web::WindowAttributesExtWebSys;

            const CANVAS_ID: &str = "canvas";

            let window = wgpu::web_sys::window().unwrap_throw();
            let document = window.document().unwrap_throw();
            let canvas = document.get_element_by_id(CANVAS_ID).unwrap_throw();

            let canvas_element = canvas.unchecked_into();

            window_attributes = window_attributes.with_canvas(Some(canvas_element));
        }

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.state = Some(pollster::block_on(state::State::new(window.clone())).unwrap());
        }

        #[cfg(target_arch = "wasm32")]
        {
            if let Some(proxy) = self.proxy.take() {
                wasm_bindgen_futures::spawn_local(async move {
                    assert!(
                        proxy
                            .send_event(State::new(window).await.expect("Failed to create state"))
                            .is_ok()
                    );
                });
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

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
