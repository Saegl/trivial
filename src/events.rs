use crate::state::BrowserState;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

pub fn event_handler(
    event_pump: &mut EventPump,
    state: &mut BrowserState,
) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => state.should_close = true,
            Event::DropFile {
                timestamp: _,
                window_id: _,
                filename,
            } => println!("{}", filename),
            Event::MouseMotion {
                timestamp: _,
                window_id: _,
                which: _,
                mousestate: _,
                x: _,
                y: _,
                xrel: _,
                yrel: _,
            } => (),
            Event::MouseWheel {
                timestamp: _,
                window_id: _,
                which: _,
                x: _,
                y,
                direction: _,
            } => {
                state.y_scroll += y * 30;
            }
            Event::Window {
                timestamp: _,
                window_id: _,
                win_event: _,
            } => (),
            event => println!("Unhandled {:?}", event),
        }
    }
}
