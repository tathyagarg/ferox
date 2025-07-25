use cfg_if::cfg_if;
use std::collections::HashMap;
use std::sync::Mutex;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use crate::{application::setup_keybindings, inputs::event_handler::event_handlers, state::State};

mod application;
mod inputs;
mod state;

fn common(#[cfg(target_arch = "wasm32")] event_loop: EventLoop<State>) {
    // let event_loop = EventLoop::new().unwrap();
    // event_loop.set_control_flow(ControlFlow::Poll);
    //
    #[cfg(not(target_arch = "wasm32"))]
    let event_loop = EventLoop::with_user_event().build().unwrap();

    event_handlers.get_or_init(|| Mutex::new(HashMap::new()));
    let mut app = application::App::new(
        #[cfg(target_arch = "wasm32")]
        &event_loop,
    );
    setup_keybindings(&mut app);

    #[cfg(not(target_arch = "wasm32"))]
    {
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut app).unwrap();
    }
}

fn run() {
    env_logger::init();

    let event_loop: EventLoop<State> = EventLoop::with_user_event().build().unwrap();

    #[cfg_attr(
        not(target_arch = "wasm32"),
        expect(unused_mut, reason = "wasm32 reassigns to specify canvas")
    )]

    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            wasm_bindgen_futures::spawn_local(async move {
                common(event_loop)
            })
        } else {
            common()
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn run_web() {
    console_error_panic_hook::set_once();
    run();
}
