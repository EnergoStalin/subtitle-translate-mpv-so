#include <libavformat/avformat.h>
#include <stddef.h>

#include "stmp.h"

AVStream *stmp_pick_stream_by_metadata(AVFormatContext *context) {
  printf("streams=%d\n", context->nb_streams);
  if (context->nb_streams == 1) {
    return context->streams[0];
  }

  for(size_t i = 0; i < context->nb_streams; i++) {
    AVStream *stream = context->streams[i];
    const AVDictionaryEntry *e;
    while((e = av_dict_iterate(stream->metadata, e))) {
      printf("key=%s, value=%s\n", e->key, e->value);
      if (stmp_metadata_stream_picker(e->value, e->value)) {
        return stream;
      }
    }
  }

  return NULL;
}