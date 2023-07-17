use core_foundation::base::CFTypeRef;
use core_foundation::dictionary::CFDictionary;
use core_foundation::string::CFString;
use core_graphics::{event, window};

fn main() -> Result<(), ()> {
    let windows = window::copy_window_info(
        window::kCGWindowListOptionOnScreenOnly,
        window::kCGNullWindowID,
    )
    .ok_or(())?;
    dbg!(&windows);

    // use core_foundation::
    for item in &windows {
        dbg!(&item);
        let a = unsafe { std::mem::transmute::<_, CFDictionary<CFString, CFTypeRef>>(item) };
        dbg!(&a);
        let m = unsafe { a.get(window::kCGWindowOwnerName) }.cast::<&str>();
        dbg!(unsafe { m.as_ref().unwrap() });
    }

    Ok(())
}
