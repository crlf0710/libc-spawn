
cfg_if! {
    if #[cfg(any(target_os = "linux"))] {
        mod linux;
        pub use self::linux::*;
    }
}
