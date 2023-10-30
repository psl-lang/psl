#pragma once
#include <unistd.h>
#include <stdbool.h>

#include "typedef.h"

void __sys_write(u8 *buf, usize len)
{
    write(1, buf, len);
}

void __write_u8(u8 i)
{
}

void __write_i32(i32 i)
{
    bool is_negative = i < 0;
    if (is_negative)
    {
        i = -i;
    }
    u8 buf[20];
    usize offset = 20;
    while (i > 0)
    {
        offset -= 1;
        buf[offset] = i % 10;
        i /= 10;
    }
    if (is_negative)
    {
        offset -= 1;
        buf[offset] = '-';
    }
    __sys_write((buf + offset), 20 - offset);
}
