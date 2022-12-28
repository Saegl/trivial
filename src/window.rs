use sdl2::video::Window;
use sdl2::Sdl;
const WINDOW_TITLE: &'static str = "Trivial Browser";

pub fn build_window(sdl_context: &Sdl) -> Window {
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(WINDOW_TITLE, 1200, 720)
        // .allow_highdpi()
        // .borderless()
        .resizable()
        .position_centered()
        .build()
        .unwrap();
    window
}
