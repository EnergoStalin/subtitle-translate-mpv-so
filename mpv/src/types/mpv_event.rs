use std::os::raw::c_void;

use crate::types::MpvError;

#[repr(C)]
#[derive(PartialEq, Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum MpvEventId {
  Shutdown = 1,
  FileLoaded = 8,
  PropertyChange = 22,
}

#[repr(C)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct MpvEvent {
  pub event_id: MpvEventId,
  pub error: MpvError,
  pub reply_userdata: u64,
  pub data: *const c_void,
}