use criterion::{black_box, criterion_group, Criterion};

use ffimage::{
    color::{Bgr, Gray, Rgb},
    iter::{ColorConvertExt, PixelsExt, WriteExt},
};

pub fn rgb_to_bgr(c: &mut Criterion) {
    let resolutions = [(640, 480), (1280, 720)];

    for res in resolutions {
        let rgb = vec![10; res.0 * res.1 * 3];
        let mut bgr = vec![10; res.0 * res.1 * 3];

        c.bench_function(&format!("Rgb[u8] -> Bgr[u8] ({}x{})", res.0, res.1), |b| {
            b.iter(|| {
                rgb.iter()
                    .copied()
                    .pixels::<Rgb<u8>>()
                    .colorconvert::<Bgr<u8>>()
                    .write(black_box(&mut bgr))
            })
        });
    }
}

pub fn rgb_to_gray(c: &mut Criterion) {
    let resolutions = [(640, 480), (1280, 720)];

    for res in resolutions {
        let rgb = vec![10; res.0 * res.1 * 3];
        let mut gray = vec![10; res.0 * res.1 * 1];

        c.bench_function(&format!("Rgb[u8] -> Gray[u8] ({}x{})", res.0, res.1), |b| {
            b.iter(|| {
                rgb.iter()
                    .copied()
                    .pixels::<Rgb<u8>>()
                    .colorconvert::<Gray<u8>>()
                    .write(black_box(&mut gray))
            })
        });
    }
}

criterion_group!(benches, rgb_to_bgr, rgb_to_gray);
