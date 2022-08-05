use std::path::Path;

use image::{ImageBuffer, Rgb, RgbImage};
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
        let image = RgbImage::from_pixel(w, h, Rgb([255u8, 255u8, 255u8]));
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
        let mut chars = std::str::from_utf8(&bytes).unwrap().chars();

        // Write text to the image
        let mut word = String::new();
        while let Some(char) = chars.next() {
            match char {
                // Handle line endings
                '\r' | '\n' => {
                    if char == '\r' {
                        chars.next();
                    }
                    if !self.write(&word) {
                        self.writeln();
                    }
                    word.clear();
                },

                // Replace tabs with 4 spaces
                '\t' => self.line.push_str("    "),

                // Write out word to image
                ' ' => {
                    self.write(&word);
                    word.clear();
                },

                // Append to word
                _ => word.push(char),
            }
        }

        // Final flush
        self.write(&word);

        self.image.save(Path::new("test.png")).unwrap();
    }

    // Intelligently write the value to the image spacing and wrapping as needed.
    // * if nothing is given nothing is written
    // * queues data until a line is filled before writing
    // * returns true if the line was wrote out to the image
    fn write(&mut self, value: &str) -> bool {
        let mut flushed = false;
        if !value.is_empty() {
            // Inject extra char to account for size of space joining pieces.
            // Using a char other than a space as the space seems to get trimmed off.
            let value_w = self.text_width(&("*".to_string() + value));
            let line_w = self.text_width(&self.line);
            if line_w + value_w > self.image.width() as i32 - self.margin * 2 {
                self.writeln();
                flushed = true;
            }
            self.line.push(' ');
            self.line.push_str(value);
        }
        flushed
    }

    // Write out the internal line to the image and advance to the newline
    fn writeln(&mut self) {
        if !self.line.is_empty() {
            draw_text_mut(
                &mut self.image,
                self.font_color,
                self.x,
                self.y,
                self.font_scale,
                &self.font,
                &self.line,
            );
            self.line.clear();
        }
        self.newline();
    }

    // Simulate a newline by moving y down a line and resetting x
    fn newline(&mut self) {
        self.y += self.font_size as i32;
        self.x = self.margin;
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
