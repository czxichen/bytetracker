#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(improper_ctypes)]

mod inner {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use inner::{cv_Rect_, Object};

pub struct Tracing {
    ins: *mut inner::BYTETracker,
}

impl Tracing {
    pub fn new(frame_rate: i32, track_buffer: i32) -> Self {
        return Tracing {
            ins: unsafe { inner::create_bt_tracker(&frame_rate, &track_buffer) },
        };
    }

    pub fn update(&mut self, objects: &[inner::Object]) -> Vec<inner::tracing_ret> {
        let mut size = 0;
        let mut rets = Vec::new();
        let mut output = [inner::tracing_ret::default(); 256];

        unsafe {
            inner::update_bt_tracker(
                self.ins,
                objects.as_ptr(),
                objects.len() as i32,
                output.as_mut_ptr(),
                &mut size,
            )
        };
        for idx in 0..size {
            rets.push(output[idx as usize]);
        }
        return rets;
    }
}

impl Drop for Tracing {
    fn drop(&mut self) {
        unsafe { inner::destroy_bt_tracker(self.ins) };
    }
}
