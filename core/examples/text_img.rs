use std::path::Path;

use image::{imageops::colorops, ImageBuffer, Pixel, Rgb, RgbImage, Rgba, RgbaImage};
use imageproc::drawing::{draw_text_mut, text_size};
use macroquad::prelude::*;
use rusttype::{Font, Scale};

pub struct TextImage {
    image: ImageBuffer<Rgb<u8>, Vec<u8>>, // Image buffer to write to
    font: Font<'static>,                  // Font to use for text
    font_size: f32,                       // Font size to use for text
    font_scale: Scale,                    // Font scale to use for text
    font_color: Rgb<u8>,                  // Font color to use for text
    margin: i32,                          // Margin around page
    line: String,                         // Current line already wrote out
    x: i32,                               // Current x location to write to
    y: i32,                               // Current y location to write to
}

impl TextImage {
    // Create a new text image
    pub fn new() -> Self {
        let margin = 20;
        let font_size = 18.0;
        let font = Vec::from(include_bytes!("assets/DejaVuSans.ttf") as &[u8]);
        let font = Font::try_from_vec(font).unwrap();
        let font_scale = Scale { x: font_size, y: font_size };
        let (w, h) = (screen_width() as u32, screen_height() as u32);
        let mut image = RgbImage::from_pixel(w, h, Rgb([255u8, 255u8, 255u8]));
        Self {
            image,
            font,
            font_size,
            font_scale,
            font_color: Rgb([0u8, 0u8, 0u8]),
            margin,
            line: String::new(),
            x: margin,
            y: margin,
        }
    }

    // Load the given file and write it to the text image
    pub async fn load_file(&mut self, path: &str) {
        let bytes = load_file(path).await.expect("Couldn't load file");
        let mut chars = std::str::from_utf8(&bytes).unwrap().chars().peekable();

        // Write text to the image
        let (w, _h) = (screen_width(), screen_height());
        let mut line = String::new();
        let mut word = String::new();
        let mut partial_word = String::new();
        while let Some(mut char) = chars.next() {
            match char {
                // Handle windows line endings
                '\r' => {
                    if chars.peek().is_some() {
                        char = chars.next().unwrap();
                        self.newline();
                    }
                },
                // Replace tabs with 4 spaces
                '\t' => line.push_str("    "),

                // Ignore newlines
                '\n' => {},

                // Split words on spaces
                ' ' => {
                    partial_word.push(' ');
                    word = partial_word.clone();
                    partial_word.clear();
                },

                // All other chars add add to word
                _ => partial_word.push(char),
            }

            // Determine how much realestate the text will require
            let w1 = self.text_width(&line) as f32;
            let mut w2 = 0.;
            let chunk2 = line.clone() + &word;
            if !word.is_empty() {
                w2 = self.text_width(&chunk2) as f32;
            }

            // Write a line out once max size is hit or no more text is available
            if w1 > w - self.margin * 2. || char == '\n' || chars.peek().is_none() {
                self.write(&line);
                self.y += self.font_size as i32;
                line.clear();
            }
        }

        let _ = self.image.save(Path::new("test.png")).unwrap();
    }

    // Write the given value to the image
    pub fn write(&mut self, value: &str) {
        draw_text_mut(&mut self.image, self.font_color, self.x, self.y, self.font_scale, &self.font, value);
        self.line.push_str(value);
    }

    // Simulate a newline by moving y down a line
    pub fn newline(&mut self) {
        self.y += self.font_size as i32;
    }

    // Intelligently wrap as needed while writing out the given value
    pub fn write_wrap(&mut self, value: &str) {
        let chunk = self.line.clone() + value;
        if self.text_width(&self.line) > self.image.width() as i32 - self.margin {
            self.newline();
            self.line.clear();
            self.write(value);
        }
        self.newline()
    }

    // Calculate the width of the given text based on font and scale
    fn text_width(&self, text: &str) -> i32 {
        let (w, _) = text_size(self.font_scale, &self.font, text);
        w
    }
}

#[macroquad::main("Text Image example")]
async fn main() {
    let mut img = TextImage::new();
    img.load_file("examples/assets/example.txt").await;

    // Convert the image to a texture
    // let img = RgbaImage::from_raw(200, 200, img.to_raw())
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
