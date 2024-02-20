extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem");

    // Parameters are: title, width, height
    let window = video_subsystem.window("Tetris", 800, 600)
        .position_centered() // to put it in the middle of the screen
        .build() // to create the window
        .expect("Failed to create window");

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync() // To enable v-sync.
        .build()
        .expect("Couldn't get window's canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    // To make things easier to read, we'll create a constant which will be the texture's size.
    const TEXTURE_SIZE: u32 = 32;

    // We create a texture with a 32x32 size.
    let mut square_texture: Texture =
        texture_creator.create_texture_target(None, TEXTURE_SIZE, TEXTURE_SIZE)
            .expect("Failed to create a texture");

    // We use the canvas to draw into our square texture.
    canvas.with_texture_canvas(&mut square_texture, |texture| {
        // We set the draw color to green.
        texture.set_draw_color(Color::RGB(0, 255, 0));
        // Draw your content here using SDL2 functions.
        // For brevity, this part is omitted in the provided excerpt.
    });

    // The rest of your main function, including the event loop and additional logic.
    // [...]
}
