use core_foundation::base::ToVoid;
use core_foundation::dictionary::{CFDictionary, CFDictionaryGetValueIfPresent, CFDictionaryRef};
use core_foundation::string::{kCFStringEncodingUTF8, CFString, CFStringGetCStringPtr};
use core_foundation::{base::CFTypeRef, string::CFStringRef};
use core_graphics::{event, window};
use std::ffi::{c_void, CStr};

fn main() -> Result<(), ()> {
    let windows = window::copy_window_info(window::kCGWindowListOptionAll, window::kCGNullWindowID)
        .ok_or(())?;
    dbg!(&windows);

    let mut count = 1;
    // use core_foundation::
    for item in windows.iter() {
        let a = unsafe { std::mem::transmute::<_, CFDictionary>(*item) };
        // let meow = **a.get(window::kCGWindowOwnerName);
        let mut value: *const c_void = std::ptr::null();
        let value = a.get(unsafe { window::kCGWindowOwnerName.to_void() });
        dbg!(*value);

        let strang = unsafe {
            CFStringGetCStringPtr(
                *value as CFStringRef,
                core_foundation::string::kCFStringEncodingUTF8,
            )
        };
        dbg!(unsafe { CStr::from_ptr(strang) });
        // if unsafe {
        //     CFDictionaryGetValueIfPresent(a, window::kCGWindowOwnerName.to_void(), &mut value)
        // } == 1
        // {
        //     let strang = unsafe {
        //         CFStringGetCStringPtr(
        //             value as CFStringRef,
        //             core_foundation::string::kCFStringEncodingUTF8,
        //         )
        //     };

        //     if !strang.is_null() {
        //         dbg!(unsafe { CStr::from_ptr(strang) });
        //     }
        //     // dbg!(unsafe { *(value as CFStringRef) });
        // };
        // dbg!(unsafe { a.offset(window::kCGWindowOwnerName) });
        // dbg!(*item);
        // let a = unsafe { std::mem::transmute::<_, CFDictionary<CFString, CFTypeRef>>(item) };
        // let m = unsafe { a.get(window::kCGWindowOwnerName) };

        // dbg!(unsafe { dbg!(*m) });
        dbg!(count);
        count += 1;
    }

    Ok(())
}
