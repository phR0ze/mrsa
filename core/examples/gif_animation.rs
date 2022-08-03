// Display an animated GIF with macroquad
use macroquad::prelude::*;
use rgb::ComponentBytes;

#[macroquad::main("GIF Animation example")]
async fn main() {
    let mut animation = Gif::load("examples/assets/ferris.gif".to_string()).await;
    loop {
        if is_key_pressed(KeyCode::Space) {
            animation.toggle_paused();
        }

        clear_background(WHITE);
        animation.draw();
        animation.tick();

        next_frame().await
    }
}

/// A single frame in the animation
#[derive(Debug)]
pub struct Frame {
    texture: Texture2D, // Macroquad texture
    delay: f32,         // How many seconds the frame should show before advancing
}

// All data for displaying an animated gif using Macroquad
pub struct Gif {
    frames: Vec<Frame>, // Frames to show
    pub width: u16,
    pub height: u16,
    current_frame: usize,
    elapsed_time: f32,
    paused: bool,
}

impl Gif {
    /// Load and decode a GIF file using Macroquad
    ///
    /// ```rust
    /// let mut gif = Gif::load("filename.gif").await;
    /// ```
    pub async fn load(filename: String) -> Self {
        let file_bytes = load_file(&filename).await.expect("Couldn't load file");
        Self::from_bytes(&file_bytes)
    }

    /// Instantiate a new `Gif` from bytes
    ///
    /// ```rust
    /// let bytes: [u8] = ...
    /// let mut gif = Gif::from_bytes(&bytes);
    /// ```
    pub fn from_bytes(file_bytes: &[u8]) -> Gif {
        let (frames, width, height) = Self::decode_gif(&file_bytes);
        Self { frames, width, height, current_frame: 0, elapsed_time: 0., paused: false }
    }

    // Use the gif and gif-dispose crates to decode the gif
    fn decode_gif(file: &[u8]) -> (Vec<Frame>, u16, u16) {
        let mut options = gif::DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::Indexed);
        let mut decoder = options.read_info(&*file).unwrap();
        let mut screen = gif_dispose::Screen::new_decoder(&decoder);

        let mut frames: Vec<Frame> = Vec::new();
        while let Some(frame) = decoder.read_next_frame().unwrap() {
            screen.blit_frame(&frame).expect("Couldn't blit frame");
            let (pixels, frame_width, frame_height) = screen.pixels.as_contiguous_buf();
            frames.push(Frame {
                texture: Texture2D::from_rgba8(frame_width as u16, frame_height as u16, pixels.as_bytes()),
                delay: frame.delay as f32 / 100.,
            });
        }
        (frames, decoder.width(), decoder.height())
    }

    fn pos_x(&self) -> f32 {
        screen_width() / 2. - self.width as f32 / 2.
    }

    fn pos_y(&self) -> f32 {
        screen_height() / 2. - self.height as f32 / 2.
    }

    /// Draw the texture of the current frame at the middle of the screen.
    ///
    /// ```rust
    /// gif_animation.draw();
    /// ```
    pub fn draw(&self) {
        self.draw_at(self.pos_x(), self.pos_y());
    }

    /// Draw the texture of the current frame at given X/Y position.
    ///
    /// ```rust
    /// gif_animation.draw_at(42.0, 47.0);
    /// ```
    pub fn draw_at(&self, pos_x: f32, pos_y: f32) {
        draw_texture(self.frame().texture, pos_x, pos_y, WHITE);
    }

    /// Update method that needs to be called in the loop to advance to next frame
    /// when necessary.
    ///
    /// ```rust
    /// gif_animation.tick();
    /// ```
    pub fn tick(&mut self) {
        if !self.paused {
            self.elapsed_time += get_frame_time();
            if self.elapsed_time > self.frame().delay {
                self.advance_frame();
            }
        }
    }

    /// Toggle whether the animation should be playing or be paused.
    ///
    /// ```rust
    /// gif_animation.toggle_paused();
    /// ```
    pub fn toggle_paused(&mut self) {
        self.paused ^= true;
    }

    fn frame(&self) -> &Frame {
        self.frames.get(self.current_frame).unwrap()
    }

    /// Advance the frame restarting when it hits the end
    fn advance_frame(&mut self) {
        self.current_frame = if self.current_frame == self.frames.len() - 1 { 0 } else { self.current_frame + 1 };
        self.elapsed_time = 0.0;
    }
}
