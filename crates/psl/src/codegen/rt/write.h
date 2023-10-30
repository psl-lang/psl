#pragma once
#include <unistd.h>
#include <stdbool.h>

#include "typedef.h"

void __sys_write(c8 *buf, usize len)
{
    write(1, buf, len);
}

void __write_i32(i32 i)
{
    bool is_negative = i < 0;
    if (is_negative)
    {
        i = -i;
    }
    c8 buf[20];
    usize offset = 20;
    while (i > 0)
    {
        offset -= 1;
        buf[offset] = '0' + (i % 10);
        i /= 10;
    }
    if (is_negative)
    {
        offset -= 1;
        buf[offset] = '-';
    }
    __sys_write((buf + offset), 20 - offset);
}
