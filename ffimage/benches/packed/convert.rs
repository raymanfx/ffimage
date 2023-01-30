use criterion::{black_box, criterion_group, Criterion};

use ffimage::color::*;
use ffimage::convert::Convert;
use ffimage::packed::Image;

pub fn rgb_to_bgr(c: &mut Criterion) {
    let resolutions = [(640, 480), (1280, 720)];

    for res in resolutions {
        let rgb = Image::<Rgb<u8>, Vec<u8>>::new(res.0, res.1, 0u8);
        let mut bgr = Image::<Rgb<u8, 2, 1, 0>, _>::new(res.0, res.1, 0u8);
        c.bench_function(&format!("Rgb[u8] -> Bgr[u8] ({}x{})", res.0, res.1), |b| {
            b.iter(|| rgb.convert(black_box(&mut bgr)))
        });
    }
}

pub fn rgb_to_gray(c: &mut Criterion) {
    let resolutions = [(640, 480), (1280, 720)];

    for res in resolutions {
        let rgb = Image::<Rgb<u8>, Vec<u8>>::new(res.0, res.1, 0u8);
        let mut gray = Image::<Gray<u8>, _>::new(res.0, res.1, 0u8);
        c.bench_function(&format!("Rgb[u8] -> Gray[u8] ({}x{})", res.0, res.1), |b| {
            b.iter(|| rgb.convert(black_box(&mut gray)))
        });
    }
}

criterion_group!(benches, rgb_to_bgr, rgb_to_gray);
