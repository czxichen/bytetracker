#include "BYTETracker.h"

#ifndef BYTETRACK_H
#define BYTETRACK_H

#ifdef __cplusplus

struct tracing_ret
{
    int tracing_id;
    float x;
    float y;
    float w;
    float h;
};

extern "C"
{
#endif
    BYTETracker *create_bt_tracker(const int &frame_rate, const int &track_buffer);

    void update_bt_tracker(BYTETracker *tracing, const Object *objects, int object_count, tracing_ret rets[256], int *tracing_ret_out);

    void destroy_bt_tracker(BYTETracker *tracing);

#ifdef __cplusplus
}
#endif

#endif