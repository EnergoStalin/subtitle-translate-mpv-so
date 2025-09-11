use std::os::raw::{c_char, c_void};

use crate::mpv::api::mpv_str::MpvStr;
use crate::mpv::ffi::{
  mpv_free, mpv_get_property, mpv_get_property_string, mpv_observe_property, mpv_set_property, mpv_wait_event
};
use crate::mpv::types::{MpvError, MpvEvent, MpvFormat, MpvHandle, ToRawMpv};

pub mod mpv_str;

pub fn free(ptr: *const c_void) {
  unsafe { mpv_free(ptr) }
}

pub fn get_property_string<'a>(handle: MpvHandle, name: *const c_char) -> MpvStr<'a> {
  unsafe { mpv_get_property_string(handle, name).into() }
}

pub fn wait_event(handle: MpvHandle, timeout: f64) -> *const MpvEvent {
  unsafe { mpv_wait_event(handle, timeout) }
}

pub fn observe_property<T>(handle: MpvHandle, reply_userdata: u64, name: *const c_char) -> MpvError
where
  T: ToRawMpv,
{
  unsafe { mpv_observe_property(handle, reply_userdata, name, T::FORMAT) }
}

pub fn observe_property_any(handle: MpvHandle, reply_userdata: u64, name: *const c_char) -> MpvError {
  unsafe {
    mpv_observe_property(handle, reply_userdata, name, MpvFormat::MpvFormatNone)
  }
}

pub fn observe_property_string(handle: MpvHandle, reply_userdata: u64, name: *const c_char) -> MpvError {
  unsafe {
    mpv_observe_property(handle, reply_userdata, name, MpvFormat::MpvFormatString)
  }
}

pub fn get_property<T>(handle: MpvHandle, name: &str) -> Result<T, MpvError>
where
  T: ToRawMpv
{
  unsafe {
    let out: T = std::mem::zeroed();
    let ret = mpv_get_property(handle, name.as_ptr() as *const c_char, T::FORMAT, out.to_raw());
    if ret != MpvError::MpvErrorSuccess {
      return Err(ret);
    }

    Ok(out)
  }
}

pub fn set_property<T>(handle: MpvHandle, name: *const c_char, value: T) -> MpvError
where
  T: ToRawMpv,
{
  unsafe {
    mpv_set_property(handle, name, T::FORMAT, value.to_raw())
  }
}