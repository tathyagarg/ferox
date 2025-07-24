use log::info;
use std::collections::HashMap;
use std::sync::Mutex;
use winit::event_loop::{ControlFlow, EventLoop};

use crate::{application::setup_keybindings, inputs::event_handler::event_handlers};

mod application;
mod inputs;
mod state;

fn handle_w(app: &mut application::App, event_loop: &winit::event_loop::ActiveEventLoop) {
    info!("Handling W key press");
    // Add your handling logic here
}

fn main() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    event_handlers.get_or_init(|| Mutex::new(HashMap::new()));
    let mut app = application::App::new();
    setup_keybindings(&mut app);

    {
        let mut handlers = event_handlers.get().unwrap().lock().unwrap();
        handlers.insert(winit::keyboard::KeyCode::KeyW, handle_w);
    }

    event_loop.run_app(&mut app).unwrap();
}
