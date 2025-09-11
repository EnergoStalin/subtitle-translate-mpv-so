use std::os::raw::c_char;

use crate::mpv::{
  api::{get_property_string, observe_property_string, wait_event},
  types::{MpvEventProperty, MpvEventPropertyData, MpvHandle},
};

mod mpv;

#[unsafe(no_mangle)]
pub extern "C" fn mpv_open_cplugin(handle: MpvHandle) -> i32 {
  observe_property_string(handle, 0, cstr!("sub-text/ass"));

  loop {
    let event = wait_event(handle, 2000.0);
    let eid = unsafe { (*event).event_id };
    match eid {
      mpv::types::MpvEventId::FileLoaded => {
        println!("{}", get_property_string(handle, cstr!("path")));
      }
      mpv::types::MpvEventId::Shutdown => return 0,
      mpv::types::MpvEventId::PropertyChange => {
        let data = unsafe { (*event).data };
        if data.is_null() { continue; }

        // let ptr = data as *const u8;
        // for i in 0..size_of::<MpvEventPropertyData>() + 1 {
        //   unsafe { println!("{}: {:x}", i, *ptr.add(i)); }
        // }
        let property = MpvEventProperty::from_ptr(data);
        println!("{}", property);
      }
    }
  }
}