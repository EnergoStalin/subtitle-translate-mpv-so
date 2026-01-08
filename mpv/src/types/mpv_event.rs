use std::{
  ffi::{c_int, c_uint, c_ulong, c_void},
  fmt::Display,
};

use enum_primitive::FromPrimitive;

use crate::{ffi::mpv_event_name, types::MpvError};

enum_from_primitive! {
#[repr(C)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum MpvEventId {
  Shutdown = 1,
  FileLoaded = 8,
  PropertyChange = 22,
  Unknown,
}
}

impl Display for MpvEventId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    unsafe { f.write_str(mpv_event_name(*self as i32).as_str()) }
  }
}

#[repr(C)]
#[derive(Debug)]
pub struct FfiMpvEvent {
  pub event_id: c_uint,
  pub error: c_int,
  pub reply_userdata: c_ulong,
  pub data: *const c_void,
}

#[derive(Debug)]
pub struct MpvEvent {
  pub event_id: MpvEventId,
  pub error: MpvError,
  pub reply_userdata: u64,
  pub data: *const c_void,
}

impl From<&FfiMpvEvent> for MpvEvent {
  fn from(value: &FfiMpvEvent) -> Self {
    MpvEvent {
      event_id: MpvEventId::from_u32(value.event_id).unwrap_or(MpvEventId::Unknown),
      error: MpvError::from_i32(value.error).unwrap_or(MpvError::MpvErrorGeneric),
      reply_userdata: value.reply_userdata,
      data: value.data,
    }
  }
}
