#include <unistd.h>
#include <spawn.h>

extern char **environ;

char** libc_spawn_helper_environ() {
    return environ;
}

int libc_spawn_helper_try_get_posix_spawn_usevfork(short* val) {
#ifdef POSIX_SPAWN_USEVFORK
    *val = POSIX_SPAWN_USEVFORK;
    return 1;
#else
    return 0;
#endif
}
