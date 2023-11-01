#include <unistd.h>
#include <stdbool.h>

#include "typedef.h"
#include "panic.h"

const usize READ_BUF_LEN = 1 << 15;
c8 read_buf[1 << 15];
usize read_off = READ_BUF_LEN;
usize read_len = READ_BUF_LEN;

void __fill_buf()
{
    read_off = 0;
    read_len = read(STDIN_FILENO, read_buf, READ_BUF_LEN);
}

c8 __peek_c8()
{
    if (read_off >= read_len)
    {
        __fill_buf();
    }
    return read_buf[read_off];
}

c8 __consume_c8()
{
    c8 result = __peek_c8();
    read_off++;
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

    bool is_negative = peek == '-';
    if (is_negative)
    {
        __consume_c8();
        peek = __peek_c8();
    }

    if ('0' > peek || peek > '9')
    {
        __sys_panic("cannot read i32", 16);
        return -1;
    }

    u32 result = 0;
    while ('0' <= peek && peek <= '9')
    {
        __consume_c8();
        result = result * 10 + peek - '0';
        peek = __peek_c8();
    }

    return is_negative ? ~result + 1 : result;
}