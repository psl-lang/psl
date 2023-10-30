#pragma once
#include <unistd.h>

#include "typedef.h"
#include "write.h"

i32 __sys_panic(u8 *msg, size_t len)
{
    __sys_write(msg, len);
    _exit(1);
    return 0;
}

i32 __sys_todo()
{
    return __sys_panic("not implemented yet\n", 20);
}