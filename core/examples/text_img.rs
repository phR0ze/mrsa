use image::{imageops::FilterType, DynamicImage};
use macroquad::prelude::*;
use rivia_vfs::prelude::*;

#[macroquad::main("Text Image example")]
async fn main() {
    // pub async fn load(path: &str) -> Self {
    //     let bytes = load_file(path).await.expect("Couldn't load file");
    //     let img = image::load_from_memory(&bytes).unwrap_or_else(|e| panic!("{}", e));
    //     let texture = Self::texturize(&img);
    //     let thumbnail = Self::texturize(&img.resize_to_fill(200, 200, FilterType::Triangle));
    //     Self { path: PathBuf::from(path), image: img, thumbnail, texture }
    // }

    // // Convert an image into a texture
    // fn texturize(img: &DynamicImage) -> Texture2D {
    //     let width = img.width() as u16;
    //     let height = img.height() as u16;
    //     let bytes = img.to_rgba8().into_raw();
    //     Texture2D::from_rgba8(width, height, &bytes)
    // }
    let texture = load_texture("examples/assets/ferris.png").await.unwrap();

    loop {
        clear_background(WHITE);

        // Center texture on the screen
        let (x, y) = (screen_width() / 2. - texture.width() / 2., screen_height() / 2. - texture.height() / 2.);

        draw_texture(texture, x, y, WHITE);
        next_frame().await
    }
}
