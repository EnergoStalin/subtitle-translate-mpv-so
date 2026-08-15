#include "packet.h"

int stmp_read_stream_packet(AVFormatContext *context, AVPacket *packet, int index) {
  int ret;

  while ((ret = av_read_frame(context, packet)) >= 0) {
    if (packet->stream_index == index)
      return 0;

    av_packet_unref(packet);
  }

  return ret;
}

size_t stmp_read_packet(
  Cookie *c,
  char *dst,
  size_t n
) {
  if (c->buf_pos >= c->buf_size)
    return 0;

  size_t available = c->buf_size - c->buf_pos;
  size_t count = available < n ? available : n;

  memcpy(
    dst,
    c->buf + c->buf_pos,
    count
  );

  c->buf_pos += count;

  if (c->buf_pos == c->buf_size) {
    c->buf = NULL;
    c->buf_pos = 0;
    c->buf_size = 0;

    av_packet_unref(c->pkt);
  }

  return count;
}

int stmp_load_packet(Cookie *c) {
  if (!c->pkt) c->pkt = av_packet_alloc();

  int ret = stmp_read_stream_packet(
    c->context,
    c->pkt,
    c->stream->index
  );

  if (ret < 0)
    return ret;

  c->buf = c->pkt->data;
  c->buf_pos = 0;
  c->buf_size = c->pkt->size;

  return 0;
}