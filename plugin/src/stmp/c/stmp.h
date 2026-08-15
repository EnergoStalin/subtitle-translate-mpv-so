#pragma once
#include <libavcodec/avcodec.h>
#include <libavcodec/codec.h>
#include <libavformat/avformat.h>

#define PROTOCOLLESS_PATH(x) (x + sizeof("stmp://") - 1)

typedef struct Cookie {
  AVFormatContext *context;
  AVStream *stream;

  AVCodecContext *codec;
  AVPacket *pkt;

  const uint8_t *buf;
  size_t buf_pos;
  size_t buf_size;

  size_t extradata_pos;
  size_t streampos;
} Cookie;

extern int stmp_metadata_stream_picker(const char *key, const char* value);

extern int stmp_process_subtitle_line(const char *line);