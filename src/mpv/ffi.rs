use std::{os::raw::{c_char, c_void}};
use crate::mpv::types::{MpvEvent, MpvFormat, MpvHandle};

unsafe extern "C" {
  pub fn mpv_wait_event(handle: MpvHandle, timeout: f64) -> *const MpvEvent;
  pub fn mpv_observe_property(handle: MpvHandle, reply_userdata: u64, name: *const c_char, format: MpvFormat) -> i32;
  pub fn mpv_free(name: *const c_void);
  pub fn mpv_get_property(handle: MpvHandle, name: *const c_char, format: MpvFormat, out: *mut c_void) -> i32;
  pub fn mpv_get_property_string(handle: MpvHandle, name: *const c_char) -> *mut c_char;
  pub fn mpv_set_property(handle: MpvHandle, name: *const c_char, format: MpvFormat, ptr: *mut c_void) -> i32;
}