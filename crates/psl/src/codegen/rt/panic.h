#pragma once

#include "typedef.h"
#include "write.h"

i32 __sys_panic(c8 *msg, size_t len)
{
    __sys_write(msg, len);
    _Exit(1);
    return 0;
}

i32 __sys_todo()
{
    return __sys_panic("not implemented yet\n", 20);
}
