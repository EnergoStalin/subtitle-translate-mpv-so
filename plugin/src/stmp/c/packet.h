#pragma once
#include <libavcodec/packet.h>
#include <libavformat/avformat.h>
#include "stmp.h"

int stmp_read_stream_packet(AVFormatContext *context, AVPacket *packet, int index);

size_t stmp_read_packet(
  Cookie *c,
  char *dst,
  size_t n
);

int stmp_load_packet(Cookie *c);