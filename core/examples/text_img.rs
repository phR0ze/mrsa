// Display a text file as an image in macroquad
use macroquad::prelude::*;
use mrsa_core::prelude::*;

// Configure window
fn window_conf() -> Conf {
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
async fn main() {
    let mut img = TextImage::new(screen_width() as u32, screen_height() as u32)
        .margin(20)
        .top_margin(30)
        .font_size(24.)
        .font("assets/fonts/Roboto-Regular.ttf")
        .await;
    img.load_file("examples/assets/example.txt").await;

    loop {
        clear_background(WHITE);

        img.draw_center();

        next_frame().await
    }
}
