use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use winit::event_loop::ActiveEventLoop;
use winit::keyboard::KeyCode;

use crate::application::App;

pub static event_handlers: OnceLock<Mutex<HashMap<KeyCode, fn(&mut App, &ActiveEventLoop) -> ()>>> =
    OnceLock::new();
