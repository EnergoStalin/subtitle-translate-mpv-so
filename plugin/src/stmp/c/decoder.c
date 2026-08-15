#include "decoder.h"

int stmp_init_decoder(Cookie *c) {
  const AVCodecParameters *par = c->stream->codecpar;

  const AVCodec *codec = avcodec_find_decoder(par->codec_id);
  if (!codec) {
    return AVERROR_DECODER_NOT_FOUND;
  }

  const AVCodec *decoder = avcodec_find_decoder(c->stream->codecpar->codec_id);

  c->codec = avcodec_alloc_context3(NULL);;
  if (!c->codec)
    return AVERROR(ENOMEM);

  int ret = avcodec_parameters_to_context(c->codec, par);
  if (ret < 0)
    return ret;

  ret = avcodec_open2(c->codec, codec, NULL);
  if (ret < 0)
    return ret;

  return 0;
}
