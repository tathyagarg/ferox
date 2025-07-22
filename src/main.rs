use std::ffi::CString;

pub mod window;

pub fn main() {
    let user_app = window::UserApp {
        height: 600,
        width: 800,
        title: CString::new("Ferox").unwrap().into_raw(),
    };

    window::run(user_app);
}
