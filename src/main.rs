use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::Duration;
use trivial::events::event_handler;
use trivial::renderer::{computer_textures, draw_nodes};
use trivial::state::BrowserState;
use trivial::window::build_window;

fn game_loop(
    mut canvas: Canvas<Window>,
    mut event_pump: EventPump,
    state: &mut BrowserState,
) {
    while !state.should_close {
        // i = (i + 1) % 255;
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        draw_nodes(state, &mut canvas);

        // let rect = Rect::new(0, 0, 500, 500);
        // canvas.set_draw_color(Color::MAGENTA);
        // canvas.filled_circle(150, 150, 150, Color::MAGENTA).unwrap();
        // let texture_creator = canvas.texture_creator();
        // texture_creator.create_texture(format, access, width, height)
        // canvas.draw_rect(rect).unwrap();
        // canvas.fill_rect(rect).unwrap();
        event_handler(&mut event_pump, state);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 6000));
    }
}

pub fn main() {
    trivial::utils::dpi_aware();
    // trivial::utils::print_all_cpu_info();
    let sdl_context = sdl2::init().unwrap();

    let window = build_window(&sdl_context);
    let canvas = window
        .into_canvas()
        .software()
        // On my machine window resize is very slow without this option
        // it's somehow related to NVIDIA GPU
        // in power-save mode (with gpu disabled) it becomes fast
        .build()
        .unwrap();
    let event_pump = sdl_context.event_pump().unwrap();
    let texture_creator = canvas.texture_creator();

    let sdl_ttf_context = sdl2::ttf::init().unwrap();
    let fonts_manager =
        trivial::fonts::import_fonts(&sdl_ttf_context);

    let source = include_str!("index.html");
    // let source = include_str!("habr.html");
    println!("Parsing started");
    let tags = trivial::html_parser::parse(source);
    dbg!(&tags);
    println!("Parsing complete");

    println!("Texture processing started");
    let textures =
        computer_textures(tags, &fonts_manager, &texture_creator);
    println!("Processed textures complete: {}", textures.len());

    let mut state = BrowserState {
        should_close: false,
        textures,
        fonts_manager,
        y_scroll: 0,
    };

    game_loop(canvas, event_pump, &mut state);
}
