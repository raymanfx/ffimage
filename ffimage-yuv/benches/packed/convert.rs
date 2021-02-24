use criterion::{criterion_group, Criterion};

use ffimage::color::*;
use ffimage::core::traits::Convert;
use ffimage::packed::Image;

use ffimage_yuv::yuv::*;
use ffimage_yuv::yuyv::*;

pub fn rgb_to_yuv(c: &mut Criterion) {
    let mem: Vec<u8> = vec![0; 640 * 480 * 3];
    let view = Image::<Rgb<u8>, _>::from_buf(&mem, 640, 480).unwrap();
    let mut buf = Image::<Yuv<u8>, _>::new(640, 480, 0u8);
    c.bench_function("Rgb[u8] -> Yuv[u8] (640x480)", |b| {
        b.iter(|| view.convert(&mut buf))
    });

    let mem: Vec<u8> = vec![0; 1280 * 720 * 3];
    let view = Image::<Rgb<u8>, _>::from_buf(&mem, 1280, 720).unwrap();
    let mut buf = Image::<Yuv<u8>, _>::new(1280, 720, 0u8);
    c.bench_function("Rgb[u8] -> Yuv[u8] (1280x720)", |b| {
        b.iter(|| view.convert(&mut buf))
    });
}

pub fn rgb_to_yuyv(c: &mut Criterion) {
    let mem: Vec<u8> = vec![0; 640 * 480 * 3];
    let view = Image::<Rgb<u8>, _>::from_buf(&mem, 640, 480).unwrap();
    let mut inter = Image::<Yuv<u8>, _>::new(640, 480, 0u8);
    let mut buf = Image::<Yuyv<u8>, _>::new(640, 480, 0u8);
    c.bench_function("Rgb[u8] -> Yuyv[u8] (640x480)", |b| {
        b.iter(|| {
            view.convert(&mut inter);
            inter.convert(&mut buf);
        })
    });

    let mem: Vec<u8> = vec![0; 1280 * 720 * 3];
    let view = Image::<Rgb<u8>, _>::from_buf(&mem, 1280, 720).unwrap();
    let mut inter = Image::<Yuv<u8>, _>::new(640, 480, 0u8);
    let mut buf = Image::<Yuyv<u8>, _>::new(1280, 720, 0u8);
    c.bench_function("Rgb[u8] -> Yuyv[u8] (1280x720)", |b| {
        b.iter(|| {
            view.convert(&mut inter);
            inter.convert(&mut buf);
        })
    });
}

criterion_group!(benches, rgb_to_yuv, rgb_to_yuyv);
