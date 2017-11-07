use libc;

cfg_if! {
    if #[cfg(target_env = "uclibc")] {
//        mod uclibc;
//        pub use self::uclibc::*;
    } else if #[cfg(target_env = "newlib")] {
//        mod newlib;
//        pub use self::newlib::*;
    } else if #[cfg(any(target_os = "linux",
                        target_os = "android",
                        target_os = "emscripten",
                        target_os = "fuchsia"))] {
        mod notbsd;
        pub use self::notbsd::*;
    } else if #[cfg(any(target_os = "macos",
                        target_os = "ios",
                        target_os = "freebsd",
                        target_os = "dragonfly",
                        target_os = "openbsd",
                        target_os = "netbsd",
                        target_os = "bitrig"))] {
        mod bsd;
        pub use self::bsd::*;
    }
}

extern {
    pub fn posix_spawn (pid: * mut libc::pid_t,
                        path: * const libc::c_char,
                        file_actions: * const posix_spawn_file_actions_t,
                        attrp: * const posix_spawnattr_t,
                        argv: * const * mut libc::c_char,
                        envp: * const * mut libc::c_char) -> libc::c_int;
    
    pub fn posix_spawnp (pid: * mut libc::pid_t,
                         file: * const libc::c_char,
                         file_actions: * const posix_spawn_file_actions_t,
                         attrp: * const posix_spawnattr_t,
                         argv: * const * mut libc::c_char,
                         envp: * const * mut libc::c_char) -> libc::c_int;

    pub fn posix_spawnattr_init (attr: * mut posix_spawnattr_t) -> libc::c_int;

    pub fn posix_spawnattr_destroy (attr: * mut posix_spawnattr_t) -> libc::c_int;
    
    pub fn posix_spawnattr_getsigdefault (attr: * const posix_spawnattr_t, sigdefault: * mut libc::sigset_t) -> libc::c_int;

    pub fn posix_spawnattr_setsigdefault (attr: * mut posix_spawnattr_t, sigdefault: * const libc::sigset_t) -> libc::c_int;
    
    pub fn posix_spawnattr_getsigmask (attr: * const posix_spawnattr_t, sigmask: * mut libc::sigset_t) -> libc::c_int;

    pub fn posix_spawnattr_setsigmask (attr: * mut posix_spawnattr_t, sigmask: * const libc::sigset_t) -> libc::c_int;

    pub fn posix_spawnattr_getflags (attr: * const posix_spawnattr_t, flags: * mut libc::c_short) -> libc::c_int;

    pub fn posix_spawnattr_setflags (attr: * mut posix_spawnattr_t, flags: libc::c_short) -> libc::c_int;

    pub fn posix_spawnattr_getpgroup (attr: * const posix_spawnattr_t, pgroup: * mut libc::pid_t) -> libc::c_int;

    pub fn posix_spawnattr_setpgroup (attr: * mut posix_spawnattr_t, pgroup: libc::pid_t) -> libc::c_int;

    pub fn posix_spawn_file_actions_init (file_actions: * mut posix_spawn_file_actions_t) -> libc::c_int;

    pub fn posix_spawn_file_actions_destroy (file_actions: * mut posix_spawn_file_actions_t) -> libc::c_int;

    pub fn posix_spawn_file_actions_addopen (file_actions: * mut posix_spawn_file_actions_t,
        fd: libc::c_int, path: * const libc::c_char, oflag: libc::c_int, mode: libc::mode_t) -> libc::c_int;

    pub fn posix_spawn_file_actions_addclose (file_actions: * mut posix_spawn_file_actions_t, fd: libc::c_int) -> libc::c_int;

    pub fn posix_spawn_file_actions_adddup2 (file_actions: * mut posix_spawn_file_actions_t, fd: libc::c_int, newfd: libc::c_int) -> libc::c_int;

}
