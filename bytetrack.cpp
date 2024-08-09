#include "bytetrack.h"
#include "BYTETracker.h"

extern "C"
{
    BYTETracker *create_bt_tracker(const int &frame_rate,
                                   const int &track_buffer)
    {
        return new BYTETracker(frame_rate, track_buffer);
    }

    void update_bt_tracker(BYTETracker *tracing, const Object *objects, int object_count, tracing_ret rets[256], int *tracing_ret_out)
    {
        const Object *obj_array = static_cast<const Object *>(objects);
        std::vector<Object> cpp_objects(obj_array, obj_array + object_count);

        std::vector<STrack> output_stracks = tracing->update(cpp_objects);
        for (decltype(output_stracks.size()) i = 0; i < output_stracks.size(); i++)
        {
            std::vector<float> tlwh = output_stracks[i].tlwh;
            rets[i] = tracing_ret{.tracing_id = output_stracks[i].track_id, .x = tlwh[0], .y = tlwh[1], .w = tlwh[2], .h = tlwh[3]};
        }
        *tracing_ret_out = output_stracks.size();
    }

    void destroy_bt_tracker(BYTETracker *tracing)
    {
        delete tracing;
    }
}
