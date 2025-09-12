use crate::mpv::{
  api::{get_property_string, observe_property_string, wait_event},
  types::{MpvProperty, MpvHandle},
};

mod mpv;

#[unsafe(no_mangle)]
pub extern "C" fn mpv_open_cplugin(handle: MpvHandle) -> i32 {
  observe_property_string(handle, 0, cstr!("sub-text/ass"));

  loop {
    let event = wait_event(handle, 2000.0);
    let eid = unsafe { (*event).event_id };

    println!("[Rust] Event: 0x{:x}", event as usize);

    match eid {
      mpv::types::MpvEventId::FileLoaded => {
        println!("{}", get_property_string(handle, cstr!("path")));
      }
      mpv::types::MpvEventId::Shutdown => return 0,
      mpv::types::MpvEventId::PropertyChange => {
        let data = unsafe { (*event).data };
        if data.is_null() { continue; }

        println!("[Rust] Property: 0x{:x}", data as usize);
        // let ptr = data as *const u8;
        // for i in 0..size_of::<MpvEventPropertyData>() + 1 {
        //   unsafe { println!("{}: {:x}", i, *ptr.add(i)); }
        // }
        let property = MpvProperty::from_ptr(data);
        println!("{}", property);
      }
    }
  }
}