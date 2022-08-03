use image::imageops::FilterType;
use image::ImageFormat;
use std::fmt;
use std::fs::File;
use std::time::{Duration, Instant};

struct Elapsed(Duration);

impl Elapsed {
    fn from(start: &Instant) -> Self {
        Elapsed(start.elapsed())
    }
}

impl fmt::Display for Elapsed {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match (self.0.as_secs(), self.0.subsec_nanos()) {
            (0, n) if n < 1000 => write!(out, "{} ns", n),
            (0, n) if n < 1000_000 => write!(out, "{} µs", n / 1000),
            (0, n) => write!(out, "{} ms", n / 1000_000),
            (s, n) if s < 10 => write!(out, "{}.{:02} s", s, n / 10_000_000),
            (s, _) => write!(out, "{} s", s),
        }
    }
}

fn main() {
    let img = image::open("examples/assets/ferris.png").unwrap();
    for &(name, filter) in [
        //("near", FilterType::Nearest), // crappy quality
        ("tri", FilterType::Triangle),   // 293 ms
        ("cmr", FilterType::CatmullRom), // 449 ms
        ("gauss", FilterType::Gaussian), // 613 ms
        ("lcz2", FilterType::Lanczos3),  // 607 ms
    ]
    .iter()
    {
        let timer = Instant::now();
        let scaled = img.resize(400, 400, filter);
        println!("Scaled by {} in {}", name, Elapsed::from(&timer));
        let mut output = File::create(&format!("test-{}.png", name)).unwrap();
        scaled.write_to(&mut output, ImageFormat::Png).unwrap();
    }

    // Fast but doesn't allow for resize FIT
    // for size in &[20_u32, 40, 100, 200, 400] {
    //     let timer = Instant::now();
    //     let scaled = img.thumbnail(*size, *size);
    //     println!("Thumbnailed to {} in {}", size, Elapsed::from(&timer));
    //     let mut output = File::create(format!("test-thumb{}.png", size)).unwrap();
    //     scaled.write_to(&mut output, ImageFormat::Png).unwrap();
    // }
}
