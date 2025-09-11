use crate::mpv::{api::{get_property_string, wait_event}, types::MpvHandle};

mod mpv;

#[unsafe(no_mangle)]
pub extern "C" fn mpv_open_cplugin(handle: MpvHandle) -> i32 {
  loop {
    let event = wait_event(handle, 2000.0);
    match unsafe { (*event).event_id } {
        mpv::types::MpvEventId::FileLoaded => println!("{}", get_property_string(handle, cstr!("filename"))),
        mpv::types::MpvEventId::Shutdown => return 0,
        mpv::types::MpvEventId::PropertyChange => {}
    }
  }
}