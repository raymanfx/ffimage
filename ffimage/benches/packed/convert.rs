use criterion::{black_box, criterion_group, Criterion};

use ffimage::color::*;
use ffimage::packed::Image;
use ffimage::traits::Convert;

pub fn rgb_to_bgr(c: &mut Criterion) {
    let mem: Vec<u8> = vec![0; 640 * 480 * 3];
    let view = Image::<Rgb<u8>, _>::from_buf(&mem, 640, 480).unwrap();
    let mut buf = Image::<Bgr<u8>, _>::new(640, 480, 0u8);
    c.bench_function("RGB[u8] -> BGR[u8] (640x480)", |b| {
        b.iter(|| view.convert(black_box(&mut buf)))
    });

    let mem: Vec<u8> = vec![0; 1280 * 720 * 3];
    let view = Image::<Rgb<u8>, _>::from_buf(&mem, 1280, 720).unwrap();
    let mut buf = Image::<Bgr<u8>, _>::new(1280, 720, 0u8);
    c.bench_function("RGB[u8] -> BGR[u8] (1280x720)", |b| {
        b.iter(|| view.convert(black_box(&mut buf)))
    });
}

pub fn rgb_to_gray(c: &mut Criterion) {
    let mem: Vec<u8> = vec![0; 640 * 480 * 3];
    let view = Image::<Rgb<u8>, _>::from_buf(&mem, 640, 480).unwrap();
    let mut buf = Image::<Gray<u8>, _>::new(640, 480, 0u8);
    c.bench_function("RGB[u8] -> Gray[u8] (640x480)", |b| {
        b.iter(|| view.convert(black_box(&mut buf)))
    });

    let mem: Vec<u8> = vec![0; 1280 * 720 * 3];
    let view = Image::<Rgb<u8>, _>::from_buf(&mem, 1280, 720).unwrap();
    let mut buf = Image::<Gray<u8>, _>::new(1280, 720, 0u8);
    c.bench_function("RGB[u8] -> Gray[u8] (1280x720)", |b| {
        b.iter(|| view.convert(black_box(&mut buf)))
    });
}

criterion_group!(benches, rgb_to_bgr, rgb_to_gray);
