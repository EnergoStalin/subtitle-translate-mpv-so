#pragma once
#include <stddef.h>
#include <stdint.h>
#include <string.h>

size_t stmp_read_buffer(
  const uint8_t *src,
  size_t *pos,
  size_t size,
  char *dst,
  size_t n
);