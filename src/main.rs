use core_foundation::dictionary::CFDictionary;
use core_foundation::string::{kCFStringEncodingUTF8, CFString, CFStringGetCStringPtr};
use core_foundation::{base::CFTypeRef, string::CFStringRef};
use core_graphics::{event, window};

fn main() -> Result<(), ()> {
    let windows = window::copy_window_info(
        window::kCGWindowListOptionOnScreenOnly,
        window::kCGNullWindowID,
    )
    .ok_or(())?;

    // use core_foundation::
    for item in &windows {
        let a = unsafe { std::mem::transmute::<_, CFDictionary<CFString, CFTypeRef>>(item) };
        let m = unsafe { a.get(window::kCGWindowOwnerName).cast::<CFStringRef>() };

        dbg!(unsafe { CFStringGetCStringPtr(*m, kCFStringEncodingUTF8) });

    }

    Ok(())
}
