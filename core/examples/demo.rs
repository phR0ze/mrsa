use image::{imageops::FilterType, DynamicImage};
use macroquad::prelude::*;
use rivia_vfs::prelude::*;

pub struct Image {
    path: PathBuf,
    image: DynamicImage,
    texture: Texture2D,
    thumbnail: Texture2D,
}
impl Image {
    pub async fn load(path: &str) -> Self {
        let bytes = load_file(path).await.expect("Couldn't load file");
        let img = image::load_from_memory(&bytes).unwrap_or_else(|e| panic!("{}", e));
        let texture = Self::texturize(&img);
        let thumbnail = Self::texturize(&img.resize_to_fill(200, 200, FilterType::Triangle));
        Self { path: PathBuf::from(path), image: img, thumbnail, texture }
    }

    // Convert an image into a texture
    fn texturize(img: &DynamicImage) -> Texture2D {
        let width = img.width() as u16;
        let height = img.height() as u16;
        let bytes = img.to_rgba8().into_raw();
        Texture2D::from_rgba8(width, height, &bytes)
    }
}

#[macroquad::main("WINDOW NAME")]
async fn main() {
    let font = load_ttf_font("examples/assets/Audiowide-Regular.ttf").await.unwrap();
    let textures = vec![
        Image::load("examples/assets/ferris.gif").await,
        Image::load("examples/assets/ferris.jpg").await,
        Image::load("examples/assets/ferris.png").await,
        Image::load("examples/assets/ferris.tiff").await,
        Image::load("examples/assets/ferris.webp").await,
    ];

    loop {
        clear_background(BLACK);

        // Allow for a 5 pixel wide border around each image
        let mut x = 0;
        let mut y = 5;
        let mut col = 0;
        for img in textures.iter() {
            col += 1;
            if col == 1 {
                x += 5;
            }
            draw_texture(
                img.thumbnail,
                x as f32,
                y as f32,
                //screen_width() / 2. - texture.width() / 2.,
                //screen_height() / 2. - texture.height() / 2.,
                WHITE,
            );

            draw_text_ex(
                &img.path.ext().unwrap(),
                x as f32,
                (y + 25) as f32,
                TextParams { font_size: 30, font, ..Default::default() },
            );
            x += 204;
            if col == 4 {
                col = 0;
                x = 0;
                y += 205;
            }
        }
        next_frame().await
    }
}
