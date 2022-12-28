use crate::fonts::FontsManager;
use crate::html_parser::Node;
use crate::state::BrowserState;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator, TextureQuery};
use sdl2::video::{Window, WindowContext};

pub fn computer_textures<'a>(
    tags: Vec<Node>,
    fonts_manager: &FontsManager,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Vec<Texture<'a>> {
    let mut textures = vec![];
    for node in tags {
        match node {
            Node::Tag(t) => match t.name.as_str() {
                "h1" => {
                    let surface = fonts_manager
                        .font_h1
                        .render(t.text().as_str())
                        .blended(Color::BLACK)
                        // .solid(Color::BLACK)
                        // .blended_wrapped(Color::BLACK, 1200)
                        .unwrap();
                    let texture = texture_creator
                        .create_texture_from_surface(&surface)
                        .unwrap();
                    textures.push(texture);
                }
                "h2" => {
                    let surface = fonts_manager
                        .font_h2
                        .render(t.text().as_str())
                        .blended(Color::BLACK)
                        // .blended_wrapped(Color::BLACK, 1200)
                        .unwrap();
                    let texture = texture_creator
                        .create_texture_from_surface(&surface)
                        .unwrap();
                    textures.push(texture);
                }
                "h3" => {
                    let surface = fonts_manager
                        .font_h3
                        .render(t.text().as_str())
                        .blended(Color::BLACK)
                        // .blended_wrapped(Color::BLACK, 1200)
                        .unwrap();
                    let texture = texture_creator
                        .create_texture_from_surface(&surface)
                        .unwrap();
                    textures.push(texture);
                }
                "p" => {
                    let surface = fonts_manager
                        .font_p
                        .render(t.text().as_str())
                        .blended(Color::BLACK)
                        // .blended_wrapped(Color::BLACK, 1200)
                        .unwrap();
                    let texture = texture_creator
                        .create_texture_from_surface(&surface)
                        .unwrap();
                    textures.push(texture);
                }
                _ => {
                    let surface = fonts_manager
                        .font_p
                        .render(t.text().as_str())
                        .blended_wrapped(Color::BLACK, 1200)
                        // .blended_wrapped(Color::BLACK, 1200)
                        .unwrap();
                    let texture = texture_creator
                        .create_texture_from_surface(&surface)
                        .unwrap();
                    textures.push(texture);
                }
            },
            Node::Text(t) => {
                let surface = fonts_manager
                    .font_p
                    .render(t.as_str())
                    .blended(Color::BLACK)
                    // .blended_wrapped(Color::BLACK, 1200)
                    .unwrap();
                let texture = texture_creator
                    .create_texture_from_surface(&surface)
                    .unwrap();
                textures.push(texture);
            }
            _ => println!("Unknown Node"),
        }
    }
    textures
}

pub fn draw_nodes(state: &BrowserState, canvas: &mut Canvas<Window>) {
    let mut y: i32 = 0;
    for texture in state.textures.iter() {
        let TextureQuery { width, height, .. } = texture.query();

        canvas
            .copy(
                &texture,
                None,
                Rect::new(0, y + state.y_scroll, width, height),
            )
            .unwrap();
        y += height as i32 + 10;
    }
}
