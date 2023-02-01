use ffimage::color::{Bgra, Gray, Rgb, Rgba};

#[test]
fn convert_rgb_to_gray() {
    let rgb = vec![Rgb::<u8>([10, 10, 10]); 10];
    let gray: Vec<Gray<u8>> = rgb
        .iter()
        .copied()
        .map(|rgb| Gray::<u8>::from(rgb))
        .collect();

    rgb.into_iter()
        .zip(gray.into_iter())
        .for_each(|(rgb, gray)| {
            // rec601 luma
            let y =
                (0.2126 * rgb[0] as f32 + 0.7152 * rgb[1] as f32 + 0.0722 * rgb[2] as f32) as u8;
            assert_eq!(gray, Gray([y]));
        });
}

#[test]
fn convert_gray_to_rgb() {
    let gray = vec![Gray::<u8>([10]); 10];
    let rgb: Vec<Rgb<u8>> = gray
        .iter()
        .copied()
        .map(|gray| Rgb::<u8>::from(gray))
        .collect();

    gray.into_iter()
        .zip(rgb.into_iter())
        .for_each(|(gray, rgb)| {
            assert_eq!(rgb, Rgb([gray[0], gray[0], gray[0]]));
        });
}

#[test]
fn convert_rgb_to_bgra() {
    let rgb = vec![Rgb::<u8>([10, 10, 10]); 10];
    let bgra: Vec<Bgra<u8>> = rgb
        .iter()
        .copied()
        .map(|rgb| Bgra::<u8>::from(rgb))
        .collect();

    rgb.into_iter()
        .zip(bgra.into_iter())
        .for_each(|(rgb, bgra)| {
            assert_eq!(bgra, Rgba::<u8, 2, 1, 0, 3>([rgb[2], rgb[1], rgb[0], 255]));
        });
}
