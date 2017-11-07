use libc;

extern {
    fn libc_spawn_helper_environ() -> * mut * const libc::c_char;
    fn libc_spawn_helper_try_get_posix_spawn_usevfork(val: * mut libc::c_short) -> libc::c_int;
}

pub fn environ() -> * mut * const libc::c_char {
    unsafe { libc_spawn_helper_environ() }
}

pub fn posix_spawn_usevfork() -> Option<libc::c_short> {
    let mut val: libc::c_short = 0;
    unsafe {
        if libc_spawn_helper_try_get_posix_spawn_usevfork(&mut val as _) != 0 {
            Some(val)
        } else {
            None
        }
    }
}