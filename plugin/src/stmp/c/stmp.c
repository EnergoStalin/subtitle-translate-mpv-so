#include "error.h"
#include "metadata.h"
#include <libavcodec/packet.h>
#include <libavcodec/avcodec.h>
#include <libavformat/avformat.h>
#include <libavformat/avio.h>
#include <libavutil/buffer.h>
#include <libavutil/error.h>
#include <mpv/client.h>
#include <mpv/stream_cb.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

#include <vect.h>

#include "packet.h"
#include "stmp.h"
#include "buffer.h"
#include "mpv.h"
#include "packet.h"
#include "decoder.h"

#if MPV_HAS_CANCEL
void stmp_stream_cb_cancel_fn(void *cookie) {
  return;
}
#endif

size_t stmp_read_extradata(
  Cookie *c,
  char *dst,
  size_t n
) {
  AVCodecParameters *par = c->stream->codecpar;

  if (c->streampos >= (size_t)par->extradata_size) {
    return 0;
  }

  size_t read = stmp_read_buffer(
    par->extradata,
    &c->extradata_pos,
    par->extradata_size,
    dst,
    n
  );

  c->streampos += read;

  return read;
}

int64_t stmp_stream_cb_read_fn(
  void *cookie,
  char *buf,
  uint64_t nbytes
) {
  Cookie *c = cookie;

  size_t remaining = nbytes;
  size_t total = 0;

  // 1. ASS header / extradata
  if (c->extradata_pos < (size_t)c->stream->codecpar->extradata_size) {
    size_t n = stmp_read_extradata(
      c,
      buf,
      remaining
    );

    total += n;
    remaining -= n;
    buf += n;

    if (remaining == 0)
      goto done;
  }

  // 2. Remaining data from current packet
  while (remaining > 0) {
    size_t n = stmp_read_packet(
      c,
      buf,
      remaining
    );

    if (n > 0) {
      total += n;
      remaining -= n;
      buf += n;
      continue;
    }

    // 3. Need another subtitle packet
    int ret = stmp_load_packet(c);

    if (ret < 0) {
      if (ret == AVERROR_EOF)
        break;

      stmp_print_averror(
        "stmp_load_packet",
        ret
      );

      return -1;
    }
  }

done:
  c->streampos += total;

  return total;
}

int64_t stmp_stream_cb_seek_fn(void *cookie, int64_t offset) {
  return MPV_ERROR_UNSUPPORTED;
}

void stmp_stream_cb_close_fn(void *cookie) {
  Cookie* c = cookie;
  avformat_close_input(&c->context);
  if (c->pkt) av_packet_free(&c->pkt);
  if (c->codec) avcodec_free_context(&c->codec);
  free(c);
}

int64_t stmp_stream_cb_size_fn(void *cookie) {
  return MPV_ERROR_UNSUPPORTED;
}

void stmp_cb_init(mpv_stream_cb_info *info) {
  info->read_fn = stmp_stream_cb_read_fn;
  info->seek_fn = stmp_stream_cb_seek_fn;
  info->close_fn = stmp_stream_cb_close_fn;
  info->size_fn = stmp_stream_cb_size_fn;
#if MPV_HAS_CANCEL
  info->cancel_fn = stmp_stream_cb_cancel_fn;
#else
  info->cancel_fn = NULL;
#endif
}

int stmp_stream_cb_open_ro_fn(void *_, char *uri, mpv_stream_cb_info *info) {
  stmp_cb_init(info);

  AVFormatContext *context = NULL;
  const char *file = PROTOCOLLESS_PATH(uri);
  int err = avformat_open_input(&context, file, NULL, NULL);

  if(err != 0) {
    stmp_print_averror("avformat_open_input", err);
    return MPV_ERROR_LOADING_FAILED;
  }

  AVStream *stream = stmp_pick_stream_by_metadata(context);
  if (!stream) {
    return MPV_ERROR_LOADING_FAILED;
  }

  Cookie* c = calloc(1, sizeof(Cookie));
  c->context = context;
  c->stream = stream;
  info->cookie = c;

  err = stmp_init_decoder(c);
  if (err != 0) {
    stmp_print_averror("stmp_init_decoder", err);
    return MPV_ERROR_LOADING_FAILED;
  }

  return MPV_ERROR_SUCCESS;
}

int stmp_register_protocol(mpv_handle *ctx) {
  return mpv_stream_cb_add_ro(ctx, "stmp", NULL, stmp_stream_cb_open_ro_fn);
}