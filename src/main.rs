use core_foundation::dictionary::CFDictionary;
use core_foundation::string::{kCFStringEncodingUTF8, CFString, CFStringGetCStringPtr};
use core_foundation::{base::CFTypeRef, string::CFStringRef};
use core_graphics::{event, window};
use std::ffi::c_void;

fn main() -> Result<(), ()> {
    let windows = window::copy_window_info(
        window::kCGWindowListOptionOnScreenOnly,
        window::kCGNullWindowID,
    )
    .ok_or(())?;

    // use core_foundation::
    for item in windows.iter() {
        dbg!(*item);
        let a = unsafe { std::mem::transmute::<_, CFDictionary<CFString, CFTypeRef>>(item) };
        let m = unsafe { a.get(window::kCGWindowOwnerName) };

        dbg!(unsafe { dbg!(*m) });
    }

    Ok(())
}
