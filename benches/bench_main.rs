use criterion::criterion_main;

mod packed;

criterion_main! {
    packed::convert::benches,
}
