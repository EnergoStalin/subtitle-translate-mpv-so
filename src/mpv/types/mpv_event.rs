use std::{fmt::Binary, os::raw::c_void};

use crate::mpv::types::MpvError;

#[repr(C)]
#[derive(PartialEq, Clone, Copy)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum MpvEventId {
  FileLoaded = 8,
  Shutdown = 1,
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
