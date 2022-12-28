use sdl2::render::Texture;

use crate::fonts::FontsManager;

pub struct BrowserState<'a> {
    pub should_close: bool,
    pub textures: Vec<Texture<'a>>,
    pub fonts_manager: FontsManager<'a>,
    pub y_scroll: i32,
}
