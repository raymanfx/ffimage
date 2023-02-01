use criterion::criterion_main;

mod convert;

criterion_main! {
    convert::benches,
}
