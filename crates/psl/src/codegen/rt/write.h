#pragma once
#include <string.h>
#include <unistd.h>
#include <stdbool.h>

#include "typedef.h"

const usize WRITE_BUF_LEN = 1 << 15;
usize write_off = 0;

void __flush(c8 *write_buf)
{
    write(STDOUT_FILENO, write_buf, write_off);
    write_off = 0;
}

void __memcpy(char *dest, const char *src, usize len)
{
    for (usize i = 0; i < len; ++i)
    {
        dest[i] = src[i];
    }
}

void __sys_write(c8 *write_buf, c8 *buf, usize len)
{
    while (len >= WRITE_BUF_LEN - write_off)
    {
        __memcpy(write_buf + write_off, buf, WRITE_BUF_LEN - write_off);
        buf += WRITE_BUF_LEN - write_off;
        len -= WRITE_BUF_LEN - write_off;
        __flush(write_buf);
    }
    __memcpy(write_buf + write_off, buf, len);
    write_off += len;
}

void __write_i32(c8 *write_buf, i32 i)
{
    bool is_negative = i < 0;
    if (is_negative)
    {
        i = -i;
    }
    c8 buf[20] = {
        0,
    };
    usize offset = 20;
    do
    {
        offset -= 1;
        buf[offset] = '0' + (i % 10);
        i /= 10;
    } while (i > 0);
    if (is_negative)
    {
        offset -= 1;
        buf[offset] = '-';
    }
    __sys_write(write_buf, buf + offset, 20 - offset);
}
