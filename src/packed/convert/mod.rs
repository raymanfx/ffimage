cfg_if::cfg_if! {
    if #[cfg(feature = "rayon")] {
        pub mod rayon;
    } else {
        pub mod gold;
    }
}
