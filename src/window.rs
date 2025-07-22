use fermium::prelude::*;
use log::info;

#[repr(C)]
pub struct UserApp {
    pub height: u32,
    pub width: u32,

    pub title: *const c_char,
}

pub struct App {
    pub user_app: UserApp,

    pub renderer: *mut SDL_Renderer,
    pub window: *mut SDL_Window,
}

pub struct ProgramState {
    pub done: bool,
}

#[unsafe(no_mangle)]
extern "C" fn init(app: &mut App) {
    unsafe { assert_eq!(SDL_Init(SDL_INIT_EVERYTHING), 0) };

    app.window = unsafe {
        SDL_CreateWindow(
            app.user_app.title,
            SDL_WINDOWPOS_CENTERED,
            SDL_WINDOWPOS_CENTERED,
            app.user_app.width as i32,
            app.user_app.height as i32,
            (SDL_WINDOW_OPENGL | SDL_WINDOW_ALLOW_HIGHDPI).0,
        )
    };

    assert!(!app.window.is_null());

    let default_driver = -1;

    app.renderer =
        unsafe { SDL_CreateRenderer(app.window, default_driver, SDL_RENDERER_ACCELERATED.0) };

    assert!(!app.renderer.is_null());
}

#[unsafe(no_mangle)]
pub extern "C" fn run(user_app: UserApp) {
    let mut app = App {
        user_app,
        renderer: std::ptr::null_mut(),
        window: std::ptr::null_mut(),
    };

    init(&mut app);

    let mut ps = ProgramState { done: false };

    while !ps.done {
        update(&mut ps);
        render(app.renderer, &ps);
    }

    unsafe { SDL_Quit() };
}

fn update(ps: &mut ProgramState) {
    let mut event = SDL_Event::default();
    let pending_events = 0 < unsafe { SDL_PollEvent(&mut event) };

    if pending_events {
        let event_type = unsafe { event.type_ };
        if event_type == SDL_QUIT {
            ps.done = true;
        } else if event_type == SDL_KEYDOWN {
            let key = unsafe { event.key.keysym.sym };
            if key == SDLK_ESCAPE {
                ps.done = true;
            } else {
                info!("Key pressed: {:?}", key);
            }
        } else if event_type == SDL_MOUSEMOTION {
            let mouse_x = unsafe { event.motion.x };
            let mouse_y = unsafe { event.motion.y };
            info!("Mouse moved to: ({}, {})", mouse_x, mouse_y);
        } else if event_type == SDL_MOUSEBUTTONDOWN {
            let button = unsafe { event.button.button };
            let mouse_x = unsafe { event.button.x };
            let mouse_y = unsafe { event.button.y };
            info!(
                "Mouse button {:?} pressed at: ({}, {})",
                button, mouse_x, mouse_y
            );
        }
    }
}

fn render(_renderer: *mut SDL_Renderer, _ps: &ProgramState) {}
