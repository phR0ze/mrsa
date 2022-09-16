use macroquad::{prelude::*, ui::root_ui};
use mrsa_core::prelude::*;

// Configure window
fn window_conf() -> Conf {
    Conf {
        window_title: "Text Image example".to_string(),
        fullscreen: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    //let mut img = TextImage::new(screen_width() as u32, screen_height() as u32);
    //img.load_file("core/examples/assets/example.txt").await;

    loop {
        clear_background(WHITE);

        // img.draw_center();
        let (x, y) = (screen_width() / 2., screen_height() / 2.);
        draw_text("IT WORKS!", x, y, 50.0, DARKGRAY);
        root_ui().label(None, "hello: ");
        if root_ui().button(None, "Start") {
            println!("pushed");
        }

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad").show(egui_ctx, |ui| {
                ui.label("Test");
            });
        });

        // Draw things before egui
        egui_macroquad::draw();
        // Draw things after egui

        next_frame().await
    }
}
