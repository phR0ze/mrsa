use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::{draw_text_mut, text_size};
use macroquad::prelude::*;
use rusttype::{Font, Scale};

pub struct TextImage
{
    image: ImageBuffer<Rgba<u8>, Vec<u8>>, // Image buffer to write to
    font: Font<'static>,                   // Font to use for text
    font_size: f32,                        // Font size to use for text
    font_scale: Scale,                     // Font scale to use for text
    font_color: Rgba<u8>,                  // Font color to use for text
    margin: u32,                           // Margin around page
    top_margin: Option<u32>,               // Margin for the top of the page
    line: String,                          // Current line already wrote out
    x: i32,                                // Current x location to write to
    y: i32,                                // Current y location to write to
    tx: Option<Texture2D>,                 // Cached texture to save conversion cost
}

impl TextImage
{
    // Create a new text image
    pub fn new(width: u32, height: u32) -> Self
    {
        let margin = 20;
        let font_size = 18.0;
        let font = Vec::from(include_bytes!("assets/DejaVuSans.ttf") as &[u8]);
        let font = Font::try_from_vec(font).unwrap();
        let font_scale = Scale { x: font_size, y: font_size };
        let image = RgbaImage::from_pixel(width, height, Rgba([255u8, 255u8, 255u8, 255u8]));
        Self {
            image,
            font,
            font_size,
            font_scale,
            font_color: Rgba([0u8, 0u8, 0u8, 255u8]),
            margin,
            top_margin: None,
            line: String::new(),
            x: margin as i32,
            y: margin as i32,
            tx: None,
        }
    }

    // Set the font option
    pub async fn font(mut self, path: &str) -> Self
    {
        let bytes = load_file(path).await.expect("Couldn't load font file");
        self.font = Font::try_from_vec(bytes).unwrap();
        self
    }

    // Set the font_size option
    pub fn font_size(mut self, value: f32) -> Self
    {
        self.font_size = value;
        self.font_scale = Scale { x: value, y: value };
        self
    }

    // Set the margin option
    pub fn margin(mut self, value: u32) -> Self
    {
        self.margin = value;
        self.x = value as i32;
        self.y = value as i32;
        self
    }

    // Se the top margin option
    pub fn top_margin(mut self, value: u32) -> Self
    {
        self.top_margin = Some(value);
        self.y = value as i32;
        self
    }

    // Load the given file and write it to the text image
    pub async fn load_file(&mut self, path: &str)
    {
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
                    self.write(&word);
                    self.writeln();
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

        // self.image.save(Path::new("test.png")).unwrap();
    }

    // Draw the image on the screen using macroquad
    pub fn draw(&mut self, x: f32, y: f32)
    {
        let tx = self.as_texture();
        draw_texture(tx, x, y, WHITE);
    }

    // Draw the image on the center of the screen using macroquad
    pub fn draw_center(&mut self)
    {
        let tx = self.as_texture();
        let (x, y) = (screen_width() / 2. - tx.width() / 2., screen_height() / 2. - tx.height() / 2.);
        draw_texture(tx, x, y, WHITE);
    }

    // Get the width of the image
    pub fn width(&self) -> u32
    {
        self.image.width()
    }

    // Get the height of the image
    pub fn height(&self) -> u32
    {
        self.image.height()
    }

    // Convert the image into a texture
    pub fn as_texture(&mut self) -> Texture2D
    {
        if self.tx.is_none() {
            let (w, h) = self.image.dimensions();
            self.tx = Some(Texture2D::from_rgba8(w as u16, h as u16, &self.image.as_raw()))
        }
        self.tx.unwrap()
    }

    // Intelligently write the value to the image spacing and wrapping as needed.
    // * if nothing is given nothing is written
    // * queues data until a line is filled before writing
    fn write(&mut self, value: &str)
    {
        if !value.is_empty() {
            // Inject extra char to account for size of space joining pieces.
            // Using a char other than a space as the space seems to get trimmed off.
            let value_w = self.text_width(&("*".to_string() + value));
            let line_w = self.text_width(&self.line);
            if line_w + value_w > self.image.width() as i32 - self.margin as i32 * 2 {
                self.writeln();
            }
            self.line.push(' ');
            self.line.push_str(value);
        }
    }

    // Write out the internal line to the image and advance to the newline
    fn writeln(&mut self)
    {
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
    fn newline(&mut self)
    {
        self.y += self.font_size as i32;
        self.x = self.margin as i32;
    }

    // Calculate the width of the given text based on font and scale
    fn text_width(&self, text: &str) -> i32
    {
        let (w, _) = text_size(self.font_scale, &self.font, text);
        w
    }
}

// Configure window
fn window_conf() -> Conf
{
    Conf {
        window_title: "Text Image example".to_string(),
        window_width: 1024,
        window_height: 768,
        high_dpi: true,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main()
{
    let mut img = TextImage::new(screen_width() as u32, screen_height() as u32)
        .margin(20)
        .top_margin(30)
        .font_size(24.)
        .font("examples/assets/Roboto-Regular.ttf")
        .await;
    img.load_file("examples/assets/example.txt").await;

    loop {
        clear_background(WHITE);

        img.draw_center();

        next_frame().await
    }
}
