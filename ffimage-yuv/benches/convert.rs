use criterion::{black_box, criterion_group, Criterion};

use ffimage::{
    color::Rgb,
    iter::{BytesExt, ColorConvertExt, PixelsExt},
};
use ffimage_yuv::{yuv::Yuv, yuv420::Yuv420p, yuv422::Yuv422};

pub fn yuv_to_rgb(c: &mut Criterion) {
    let resolutions = [(640, 480), (1280, 720)];

    for res in resolutions {
        let yuv = vec![10; res.0 * res.1 * 3];
        let mut rgb = vec![10; res.0 * res.1 * 3];

        c.bench_function(&format!("Yuv[u8] -> Rgb[u8] ({}x{})", res.0, res.1), |b| {
            b.iter(|| {
                yuv.iter()
                    .copied()
                    .pixels::<Yuv<u8>>()
                    .colorconvert::<Rgb<u8>>()
                    .bytes()
                    .write(black_box(&mut rgb))
            })
        });
    }
}

pub fn yuv422_to_rgb(c: &mut Criterion) {
    let resolutions = [(640, 480), (1280, 720)];

    for res in resolutions {
        let yuv422 = vec![10; res.0 * res.1 * 2];
        let mut rgb = vec![10; res.0 * res.1 * 3];

        c.bench_function(
            &format!("Yuv422[u8] -> Rgb[u8] ({}x{})", res.0, res.1),
            |b| {
                b.iter(|| {
                    yuv422
                        .iter()
                        .copied()
                        .pixels::<Yuv422<u8, 0, 2, 1, 3>>()
                        .colorconvert::<[Yuv<u8>; 2]>()
                        .flatten()
                        .colorconvert::<Rgb<u8>>()
                        .bytes()
                        .write(black_box(&mut rgb));
                })
            },
        );
    }
}

pub fn yuv420p_to_rgb(c: &mut Criterion) {
    let resolutions = [(640, 480), (1280, 720)];

    for res in resolutions {
        let yuv420p = vec![10; res.0 * res.1 * 3 / 2];
        let mut rgb = vec![10; res.0 * res.1 * 3];

        c.bench_function(
            &format!("Yuv420p[u8] -> Rgb[u8] ({}x{})", res.0, res.1),
            |b| {
                b.iter(|| {
                    Yuv420p::pack(&yuv420p, 640, 480)
                        .into_iter()
                        .colorconvert::<Rgb<u8>>()
                        .bytes()
                        .write(black_box(&mut rgb));
                })
            },
        );
    }
}

criterion_group!(benches, yuv_to_rgb, yuv422_to_rgb, yuv420p_to_rgb);
