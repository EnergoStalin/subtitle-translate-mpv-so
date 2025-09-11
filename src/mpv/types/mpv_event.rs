use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MpvEventId {
  FileLoaded = 8,
  Shutdown = 1,
  PropertyChange = 22,
}

#[repr(C)]
#[derive(Debug)]
pub struct MpvEvent {
  pub event_id: MpvEventId,
  pub error: i32,
  pub reply_userdata: u64,
  pub data: *mut c_void
}