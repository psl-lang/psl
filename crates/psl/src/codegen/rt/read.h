#include <unistd.h>
#include <stdbool.h>

#include "typedef.h"
#include "panic.h"

bool has_read = false;
c8 buf;

c8 __read_c8()
{
    c8 buf[1];
    read(STDIN_FILENO, &buf, 1);
    return buf[0];
}

c8 __peek_c8()
{
    if (!has_read)
    {
        buf = __read_c8();
        has_read = true;
    }
    return buf;
}

c8 __consume_c8()
{
    c8 result = __peek_c8();
    buf = __read_c8();
    return result;
}

void __skip_whitespace()
{
    c8 peek = __peek_c8();
    while (peek == ' ' || peek == '\n')
    {
        __consume_c8();
        peek = __peek_c8();
    }
}

i32 __read_i32()
{
    __skip_whitespace();
    c8 peek = __peek_c8();
    if ('0' > peek || peek > '9')
    {
        __sys_panic("cannot read i32", 16);
        return -1;
    }

    i32 result = 0;
    while ('0' <= peek && peek <= '9')
    {
        __consume_c8();
        result = result * 10 + peek - '0';
        peek = __peek_c8();
    }

    return result;
}