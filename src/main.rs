use log::{info, warn};
use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

mod input;
mod state;

#[derive(Default)]
struct App {
    state: Option<state::State>,
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
            WindowEvent::RedrawRequested => {
                state.get_window().request_redraw();
            }
            WindowEvent::Resized(size) => {
                // Reconfigures the size of the surface. We do not re-render
                // here as this event is always followed up by redraw request.
                state.resize(size);
            }
            WindowEvent::KeyboardInput {
                event,
                is_synthetic: false,
                ..
            } => match event.physical_key {
                PhysicalKey::Code(key_code) => {
                    info!("KeyboardInput: {:?}", key_code);
                    if let Some(action) = input::make_key_action(key_code) {
                        if let Err(e) = input::handle_action(action, self, event_loop) {
                            info!("Error handling action: {}", e);
                        }
                    }
                }
                PhysicalKey::Unidentified(..) => {
                    warn!("Unknown key pressed");
                }
            },
            _ => (),
        }

        // match event {
        //     WindowEvent::CloseRequested => {
        //         println!("The close button was pressed; stopping");
        //         event_loop.exit();
        //     }
        //     WindowEvent::RedrawRequested => {
        //         state.render();
        //         // Emits a new redraw requested event.
        //         state.get_window().request_redraw();
        //     }
        //     WindowEvent::Resized(size) => {
        //         // Reconfigures the size of the surface. We do not re-render
        //         // here as this event is always followed up by redraw request.
        //         state.resize(size);
        //     }
        //     WindowEvent::KeyboardInput {
        //         event,
        //         is_synthetic: false,
        //         ..
        //     } => {
        //         info!("KeyboardInput: {:?}", event.physical_key)
        //     }
        //     WindowEvent::MouseInput { state, button, .. } => {
        //         info!("MouseInput: {:?} {:?}", state, button);
        //     }
        //     _ => (),
        // }
    }
}

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
