use criterion::{criterion_group, criterion_main};

mod packed;

criterion_main! {
    packed::convert::benches,
}
