use std::path::Path;

use image::{imageops::colorops, Pixel, Rgb, RgbImage, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use macroquad::prelude::*;
use rusttype::{Font, Scale};

#[macroquad::main("Text Image example")]
async fn main()
{
    // Read file data
    let bytes = load_file("examples/assets/example.txt").await.expect("Couldn't load file");
    let data = std::str::from_utf8(&bytes).unwrap();

    // Create blank white image to write to
    let (w, h) = (screen_width() as u32, screen_height() as u32);
    let mut img = RgbImage::from_pixel(w, h, Rgb([255u8, 255u8, 255u8]));

    // Write the text data to the image
    let font = Vec::from(include_bytes!("assets/DejaVuSans.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();
    // let scale = Scale { x: height * 2.0, y: height };
    let scale = Scale { x: 15., y: 12. };
    let data = "Hello world";
    draw_text_mut(&mut img, Rgb([0u8, 0u8, 0u8]), 0, 0, scale, &font, data);

    // Convert the image to a texture
    // let img = RgbaImage::from_raw(200, 200, img.to_raw())
    let _ = img.save(Path::new("test.png")).unwrap();
    // let texture = Texture2D::from_rgba8(img.width() as u16, img.height() as u16,
    // &img.into_raw());

    // loop {
    //     clear_background(WHITE);

    //     // Center texture on the screen
    //     let (x, y) = (screen_width() / 2. - texture.width() / 2., screen_height() / 2. -
    // texture.height() / 2.);

    //     draw_texture(texture, x, y, BLACK);
    //     next_frame().await
    // }
}
