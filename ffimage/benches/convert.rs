use criterion::{black_box, criterion_group, Criterion};

use ffimage::color::{Gray, Rgb};

pub fn rgb_to_bgr(c: &mut Criterion) {
    let resolutions = [(640, 480), (1280, 720)];

    for res in resolutions {
        let rgb = vec![Rgb::<u8>([10, 10, 10]); res.0 * res.1];
        let mut bgr = vec![Rgb::<u8, 2, 1, 0>([0, 0, 0]); res.0 * res.1];

        c.bench_function(&format!("Rgb[u8] -> Bgr[u8] ({}x{})", res.0, res.1), |b| {
            b.iter(|| {
                rgb.iter()
                    .zip(bgr.iter_mut())
                    .for_each(|(rgb, bgr)| black_box(*bgr = (*rgb).into()))
            })
        });
    }
}

pub fn rgb_to_gray(c: &mut Criterion) {
    let resolutions = [(640, 480), (1280, 720)];

    for res in resolutions {
        let rgb = vec![Rgb::<u8>([10, 10, 10]); res.0 * res.1];
        let mut gray = vec![Gray::<u8>([0]); res.0 * res.1];

        c.bench_function(&format!("Rgb[u8] -> Gray[u8] ({}x{})", res.0, res.1), |b| {
            b.iter(|| {
                rgb.iter()
                    .zip(gray.iter_mut())
                    .for_each(|(rgb, gray)| black_box(*gray = (*rgb).into()))
            })
        });
    }
}

criterion_group!(benches, rgb_to_bgr, rgb_to_gray);
