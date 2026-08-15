#include <libavutil/error.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

void stmp_print_averror(const char *title, int64_t code) {
  static char error[256];
  int err = av_strerror(code, error, sizeof(error));
  if (err != 0) {
    strcpy("Unknown Error", error);
  }
  printf("Code: %ld, Error: %s, AVError: %s\n", code, title, error);
}