#pragma once
#include <unistd.h>
#include <stdbool.h>

#include "typedef.h"

const usize WRITE_BUF_LEN = 1 << 15;
c8 write_buf[1 << 15];
usize write_off = WRITE_BUF_LEN;

void __sys_write(c8 *buf, usize len)
{
    while (len >= WRITE_BUF_LEN - write_off)
    {
        memcpy(write_buf + write_off, buf, WRITE_BUF_LEN - write_off);
        buf += WRITE_BUF_LEN - write_off;
        len -= WRITE_BUF_LEN - write_off;
        write_off = 0;
        write(STDOUT_FILENO, write_buf, WRITE_BUF_LEN);
    }
    memcpy(write_buf + write_off, buf, len);
    write_off += len;
}

void __flush()
{
    write(STDOUT_FILENO, write_buf, write_off);
}

void __write_i32(i32 i)
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
    __sys_write(buf + offset, 20 - offset);
}
