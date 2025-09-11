use std::ffi::c_char;

use crate::mpv::api::mpv_str::MpvStr;
use crate::mpv::ffi::{mpv_get_property_string, mpv_wait_event};
use crate::mpv::types::{MpvEvent, MpvHandle};

mod mpv_str;

pub fn get_property_string(handle: MpvHandle, name: *const c_char) -> MpvStr {
  unsafe { mpv_get_property_string(handle, name).into() }
}

pub fn wait_event(handle: MpvHandle, timeout: f64) -> *const MpvEvent {
  unsafe { mpv_wait_event(handle, timeout) }
}