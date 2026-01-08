use crate::{
  api::{
    mpv_str::{MpvStr, MpvStrOwned},
    MpvHandle,
  },
  types::{FfiMpvEvent, MpvFormat},
};
use std::ffi::{c_char, c_double, c_int, c_ulong, c_void};

unsafe extern "C" {
  pub fn mpv_wait_event(handle: MpvHandle, timeout: c_double) -> *const FfiMpvEvent;
  pub fn mpv_observe_property(
    handle: MpvHandle,
    reply_userdata: c_ulong,
    name: *const c_char,
    format: MpvFormat,
  ) -> c_int;
  pub fn mpv_free(name: *const c_void);
  pub fn mpv_get_property(
    handle: MpvHandle,
    name: *const c_char,
    format: MpvFormat,
    out: *mut c_void,
  ) -> c_int;
  pub fn mpv_get_property_string(handle: MpvHandle, name: *const c_char) -> MpvStrOwned;
  pub fn mpv_set_property(
    handle: MpvHandle,
    name: *const c_char,
    format: MpvFormat,
    ptr: *const c_void,
  ) -> c_int;
  pub fn mpv_event_name(id: c_int) -> MpvStr<'static>;
  pub fn mpv_error_string(id: c_int) -> MpvStr<'static>;
}
