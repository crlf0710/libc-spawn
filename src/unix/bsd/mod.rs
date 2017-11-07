
cfg_if! {
    if #[cfg(any(target_os = "macos", target_os = "ios"))] {
        mod apple;
        pub use self::apple::*;
    }
}
