use criterion::{black_box, criterion_group, Criterion};

use ffimage::color::*;
use ffimage::convert::Convert;
use ffimage::packed::Image;

use ffimage_yuv::yuv::*;
use ffimage_yuv::yuv422::*;

pub fn rgb_to_yuv(c: &mut Criterion) {
    let resolutions = [(640, 480), (1280, 720)];

    for res in resolutions {
        let rgb = Image::<Rgb<u8>, Vec<u8>>::new(res.0, res.1, 0u8);
        let mut yuv = Image::<Yuv<u8>, _>::new(res.0, res.1, 0u8);
        c.bench_function(&format!("Rgb[u8] -> Yuv[u8] ({}x{})", res.0, res.1), |b| {
            b.iter(|| rgb.convert(black_box(&mut yuv)))
        });
    }
}

pub fn rgb_to_yuyv(c: &mut Criterion) {
    let resolutions = [(640, 480), (1280, 720)];

    for res in resolutions {
        let rgb = Image::<Rgb<u8>, Vec<u8>>::new(res.0, res.1, 0u8);
        let mut yuv = Image::<Yuv<u8>, _>::new(res.0, res.1, 0u8);
        let mut yuyv = Image::<Yuv422<u8, 0, 2, 1, 3>, _>::new(res.0, res.1, 0u8);
        c.bench_function(&format!("Rgb[u8] -> Yuyv[u8] ({}x{})", res.0, res.1), |b| {
            b.iter(|| {
                rgb.convert(black_box(&mut yuv));
                yuv.convert(black_box(&mut yuyv))
            })
        });
    }
}

criterion_group!(benches, rgb_to_yuv, rgb_to_yuyv);
