use fermium::prelude::*;

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
        if event_type == SDL_QUIT || event_type == SDL_KEYDOWN {
            ps.done = true;
        }
    }
}

fn render(renderer: *mut SDL_Renderer, _ps: &ProgramState) {
    unsafe {
        let r = ((SDL_GetTicks() / 5) % 256) as u8;
        let g = ((SDL_GetTicks() / 10) % 256) as u8;
        let b = ((SDL_GetTicks() / 15) % 256) as u8;

        SDL_SetRenderDrawColor(renderer, r, g, b, 255);
        SDL_RenderClear(renderer);
        SDL_RenderPresent(renderer);
    }
}
