use libc;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct posix_spawnattr_t {
    flags: libc::c_short,
    pgrp: libc::pid_t,
    sd: libc::sigset_t,
    ss: libc::sigset_t,
    sp: libc::sched_param,
    policy: libc::c_int,
    pad: [libc::c_int; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct _spawn_action {
    _dummy: usize
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct posix_spawn_file_actions_t {
    allocated: libc::c_int,
    used: libc::c_int,
    actions: * mut _spawn_action,
    pad: [libc::c_int; 16],
}

pub const POSIX_SPAWN_RESETIDS: libc::c_short = 0x01;
pub const POSIX_SPAWN_SETPGROUP: libc::c_short = 0x02;
pub const POSIX_SPAWN_SETSIGDEF: libc::c_short = 0x04;
pub const POSIX_SPAWN_SETSIGMASK: libc::c_short = 0x08;
pub const POSIX_SPAWN_SETSCHEDPARAM: libc::c_short = 0x10;
pub const POSIX_SPAWN_SETSCHEDULER: libc::c_short = 0x20;


pub const POSIX_SPAWN_USEVFORK: libc::c_short = 0x40;

extern {

    pub fn posix_spawnattr_getschedpolicy (attr: * const posix_spawnattr_t, schedpolicy: * mut libc::c_int) -> libc::c_int;

    pub fn posix_spawnattr_setschedpolicy (attr: * mut posix_spawnattr_t, schedpolicy: libc::c_int) -> libc::c_int;

    pub fn posix_spawnattr_getschedparam (attr: * const posix_spawnattr_t, sigmask: * mut libc::sched_param) -> libc::c_int;

    pub fn posix_spawnattr_setschedparam (attr: * mut posix_spawnattr_t, sigmask: * const libc::sched_param) -> libc::c_int;

}
