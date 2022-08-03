use macroquad::prelude::*;
use rivia_vfs::prelude::*;

// vfs::set_memfs().unwrap();
// read_write_all("file1").unwrap();
#[macroquad::main("Texture")]
async fn main() {
    let texture: Texture2D = load_texture("examples/assets/ferris.png").await.unwrap();

    loop {
        clear_background(LIGHTGRAY);
        draw_texture(
            texture,
            screen_width() / 2. - texture.width() / 2.,
            screen_height() / 2. - texture.height() / 2.,
            WHITE,
        );
        next_frame().await
    }
}
fn read_write_all<T: AsRef<Path>>(path: T) -> RvResult<()> {
    let tmpdir = assert_setup!();
    let file1 = tmpdir.mash(path);
    vfs::write_all(&file1, "this is a test")?;
    assert_eq!(vfs::read_all(&file1)?, "this is a test".to_string());
    assert_remove_all!(&tmpdir);
    Ok(())
}
