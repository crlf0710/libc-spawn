#![allow(unused_imports)]
extern crate cc;
#[macro_use]
extern crate cfg_if;

#[cfg(unix)]
fn main() {
    cfg_if! {
        if #[cfg(any(target_env = "uclibc", target_env = "newlib"))] {
            const PREDEFINITION: &'static str = "_NOT_GNU_SOURCE";
        } else {
            const PREDEFINITION: &'static str = "_GNU_SOURCE";
        }
    }

    cc::Build::new()
        .file("src/optional_const/imp.c")
        .define(PREDEFINITION, None)
        .compile("libc_spawn_helper");
}

#[cfg(not(unix))]
fn main() {}
