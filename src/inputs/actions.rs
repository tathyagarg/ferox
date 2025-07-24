use winit::{
    event::{ElementState, MouseButton},
    keyboard::KeyCode,
};

#[derive(Debug, Clone)]
pub enum Action {
    CloseWindow,
    ButtonPress(MouseButton, ElementState),
    Keyboard(KeyCode, ElementState),
}
