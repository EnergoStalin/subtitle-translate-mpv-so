#pragma once
#include <libavformat/avformat.h>

AVStream *stmp_pick_stream_by_metadata(AVFormatContext *context);