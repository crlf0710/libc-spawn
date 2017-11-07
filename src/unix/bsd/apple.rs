
use libc;

pub type posix_spawnattr_t = * const libc::c_void;
pub type posix_spawn_file_actions_t = * const libc::c_void;

