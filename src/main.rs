use image::ImageBuffer;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

// assuming 0.0 <= value <= 1.0
fn brightness(value: f64) -> u8 {
    (value * 255f64) as u8
}

fn rnd(func: fn(i64, i64) -> i64, x: i64, y: i64) -> u8 {
    brightness(SmallRng::seed_from_u64(func(x, y) as u64).gen())
}

fn main() {
    // change these lines to play around with patterns
    let func = |x: i64, y: i64| -> i64 { x ^ y };
    let zoom: u32 = 2;
    let size: u32 = 512;
    ImageBuffer::from_fn(size, size, |x, y| {
        image::Luma([rnd(
            func,
            (((x / zoom) as i64) - (size as i64 / 2)).abs(),
            (((y / zoom) as i64) - (size as i64 / 2)).abs(),
        )])
    })
    .save("out.png")
    .unwrap();
}
