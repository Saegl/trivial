use sdl2::ttf::{Font, Sdl2TtfContext};
use std::path::Path;

pub struct FontsManager<'a> {
    pub font_h1: Font<'a, 'a>,
    pub font_h2: Font<'a, 'a>,
    pub font_h3: Font<'a, 'a>,
    pub font_p: Font<'a, 'a>,
}

pub fn import_fonts(
    sdl_ttf_context: &Sdl2TtfContext,
) -> FontsManager {
    let font_h1 = sdl_ttf_context
        .load_font(Path::new("DejaVuSans-Bold.ttf"), 48)
        .unwrap();

    let font_h2 = sdl_ttf_context
        .load_font(Path::new("DejaVuSans-Bold.ttf"), 32)
        .unwrap();

    let font_h3 = sdl_ttf_context
        .load_font(Path::new("DejaVuSans-Bold.ttf"), 24)
        .unwrap();

    let font_p = sdl_ttf_context
        .load_font(Path::new("DejaVuSans.ttf"), 22)
        .unwrap();
    FontsManager {
        font_h1,
        font_h2,
        font_h3,
        font_p,
    }
}
