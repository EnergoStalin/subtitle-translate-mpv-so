#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

size_t stmp_read_buffer(
  const uint8_t *src,
  size_t *pos,
  size_t size,
  char *dst,
  size_t n
) {
  if (*pos >= size)
    return 0;

  size_t available = size - *pos;
  size_t count = available < n ? available : n;

  memcpy(dst, src + *pos, count);

  *pos += count;

  return count;
}