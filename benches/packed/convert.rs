use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ffimage::color::*;
use ffimage::core::{Convert, ImageView, TryConvert};
use ffimage::packed::{GenericImageBuffer, GenericImageFlatBuffer, GenericImageView};

pub fn rgb_to_bgr(c: &mut Criterion) {
    let mem: Vec<u8> = vec![0; 640 * 480 * 3];
    let view = GenericImageView::<Rgb<u8>>::new(&mem, 640, 480).unwrap();
    let mut buf = GenericImageBuffer::<Bgr<u8>>::new(640, 480);
    c.bench_function("RGB[u8] -> BGR[u8] (640x480)", |b| {
        b.iter(|| view.convert(black_box(&mut buf)))
    });

    let mem: Vec<u8> = vec![0; 1280 * 720 * 3];
    let view = GenericImageView::<Rgb<u8>>::new(&mem, 1280, 720).unwrap();
    let mut buf = GenericImageBuffer::<Bgr<u8>>::new(1280, 720);
    c.bench_function("RGB[u8] -> BGR[u8] (1280x720)", |b| {
        b.iter(|| view.convert(black_box(&mut buf)))
    });
}

pub fn rgb_to_gray(c: &mut Criterion) {
    let mem: Vec<u8> = vec![0; 640 * 480 * 3];
    let view = GenericImageView::<Rgb<u8>>::new(&mem, 640, 480).unwrap();
    let mut buf = GenericImageBuffer::<Gray<u8>>::new(640, 480);
    c.bench_function("RGB[u8] -> Gray[u8] (640x480)", |b| {
        b.iter(|| view.convert(black_box(&mut buf)))
    });

    let mem: Vec<u8> = vec![0; 1280 * 720 * 3];
    let view = GenericImageView::<Rgb<u8>>::new(&mem, 1280, 720).unwrap();
    let mut buf = GenericImageBuffer::<Gray<u8>>::new(1280, 720);
    c.bench_function("RGB[u8] -> Gray[u8] (1280x720)", |b| {
        b.iter(|| view.convert(black_box(&mut buf)))
    });
}

criterion_group!(benches, rgb_to_bgr, rgb_to_gray);
