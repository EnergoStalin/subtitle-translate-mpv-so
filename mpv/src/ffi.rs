use std::os::raw::{c_char, c_int, c_ulong, c_void};
use crate::{api::{mpv_str::MpvStrOwned, MpvHandle}, types::{MpvEvent, MpvFormat}};

unsafe extern "C" {
  pub fn mpv_wait_event(handle: MpvHandle, timeout: f64) -> *const MpvEvent;
  pub fn mpv_observe_property(handle: MpvHandle, reply_userdata: c_ulong, name: *const c_char, format: MpvFormat) -> c_int;
  pub fn mpv_free(name: *const c_void);
  pub fn mpv_get_property(handle: MpvHandle, name: *const c_char, format: MpvFormat, out: *mut c_void) -> c_int;
  pub fn mpv_get_property_string(handle: MpvHandle, name: *const c_char) -> MpvStrOwned;
  pub fn mpv_set_property(handle: MpvHandle, name: *const c_char, format: MpvFormat, ptr: *const c_void) -> c_int;
}