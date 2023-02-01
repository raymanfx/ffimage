use criterion::{black_box, criterion_group, Criterion};

use ffimage::color::Rgb;
use ffimage_yuv::{yuv::Yuv, yuv422::Yuv422};

pub fn rgb_to_yuv(c: &mut Criterion) {
    let resolutions = [(640, 480), (1280, 720)];

    for res in resolutions {
        let rgb = vec![Rgb::<u8>([10, 10, 10]); res.0 * res.1];
        let mut yuv = vec![Yuv::<u8>([0, 0, 0]); res.0 * res.1];

        c.bench_function(&format!("Rgb[u8] -> Yuv[u8] ({}x{})", res.0, res.1), |b| {
            b.iter(|| {
                rgb.iter()
                    .zip(yuv.iter_mut())
                    .for_each(|(rgb, yuv)| black_box(*yuv = (*rgb).into()))
            })
        });
    }
}

pub fn rgb_to_yuyv(c: &mut Criterion) {
    let resolutions = [(640, 480), (1280, 720)];

    for res in resolutions {
        let rgb = vec![Rgb::<u8>([10, 10, 10]); res.0 * res.1];
        let [mut yuv1, mut yuv2] = [Yuv::<u8>([0, 0, 0]); 2];
        let mut yuyv = vec![Yuv422::<u8, 0, 2, 1, 3>([0, 0, 0, 0]); (res.0 * res.1) / 2];

        c.bench_function(&format!("Rgb[u8] -> Yuyv[u8] ({}x{})", res.0, res.1), |b| {
            b.iter(|| {
                (rgb.iter().zip(rgb.iter().skip(1)))
                    .zip(yuyv.iter_mut())
                    .for_each(|((rgb1, rgb2), yuyv)| {
                        black_box(yuv1 = (*rgb1).into());
                        black_box(yuv2 = (*rgb2).into());
                        black_box(*yuyv = [yuv1, yuv2].into());
                    })
            })
        });
    }
}

criterion_group!(benches, rgb_to_yuv, rgb_to_yuyv);
