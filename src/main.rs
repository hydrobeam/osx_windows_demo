use block2::{Block, ConcreteBlock};
use core_foundation::base::ToVoid;
use core_foundation::dictionary::{CFDictionary, CFDictionaryGetValueIfPresent, CFDictionaryRef};
use core_foundation::string::{kCFStringEncodingUTF8, CFString, CFStringGetCStringPtr};
use core_foundation::{base::CFTypeRef, string::CFStringRef};
use core_graphics::{event, window};
use std::ffi::{c_void, CStr};

use objc2::{
    class, extern_class, msg_send, mutability,
    runtime::{Class, Object},
    ClassType,
};

use icrate::Foundation::{NSArray, NSObject, NSString};

extern_class!(
    /// An example description.
    #[derive(PartialEq, Eq, Hash)] // Uses the superclass' implementation
                                   // Specify the class and struct name to be used
    pub struct SCShareableContent;

    // Specify the superclass, in this case `NSObject`
    unsafe impl ClassType for SCShareableContent {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        // Optionally, specify the name of the class, if it differs from
        // the struct name.
        // const NAME: &'static str = "NSFormatter";
    }
);

use icrate::Foundation::NSError;
use objc2::{extern_methods, RefEncode};

// #[no_mangle]
// pub unsafe extern "C" fn wlist_basic(block: &ConcreteBlock<(i32, i32), i32>) -> i32 {
//     block.call((5, 8))
// }

// Required to bring NSPasteboard into the path of the class-resolver
#[link(name = "ScreenCaptureKit", kind = "framework")]
extern "C" {}

fn main() -> Result<(), ()> {
    let block = ConcreteBlock::new(
        |shareable_content: *const SCShareableContent, error: *const NSError| {
            if !error.is_null() {
                panic!("unable to fetch windows, make sure permissions are granted")
            }

            let windows: &NSArray = unsafe { msg_send![shareable_content, windows] };
            for window in windows.iter() {
                let ns_title: *const NSString = unsafe { msg_send![window, title] };
                // not every window has a title
                if ns_title.is_null() {
                    continue;
                }
                let utf8title = unsafe { (*ns_title).UTF8String() };
                // SAFETY: we are guaranteed a UTF8string
                let title = unsafe { CStr::from_ptr(utf8title) }.to_str().unwrap();
                dbg!(title);
            }
        },
    );

    // block
    let sc_shareable = class!(SCShareableContent);
    unsafe {
        let _: () = msg_send![
            sc_shareable,
            getShareableContentWithCompletionHandler:&block
        ];
    };
    // give the callback time to execute
    std::thread::sleep(std::time::Duration::from_secs(1));

    // unsafe { msg_send![qq, completionHandler:&block] }

    // block::Block

    // let m = unsafe { msg_send![available_content, excludingDesktopWindows, false,] };
    // let windows = window::copy_window_info(
    //     window::kCGWindowListOptionAll
    //         | window::kCGWindowListOptionExcludeDesktopElements
    //         | window::kCGWindowListOptionOnScreenOnly,
    //     // window::kCGWindowListOptionAll | window::kCGWindowListOptionOnScreenOnly,
    //     window::kCGNullWindowID,
    // )
    // .ok_or(())?;
    // dbg!(&windows);

    // let mut count = 1;
    // // use core_foundation::
    // for item in windows.iter() {
    //     let a = unsafe { std::mem::transmute::<_, CFDictionaryRef>(*item) };
    //     // let meow = **a.get(window::kCGWindowOwnerName);
    //     let mut value: *const c_void = std::ptr::null();
    //     // let value = a.get(unsafe { window::kCGWindowOwnerName.to_void() });
    //     // dbg!(*value);

    //     // let strang = unsafe {
    //     //     CFStringGetCStringPtr(
    //     //         *value as CFStringRef,
    //     //         core_foundation::string::kCFStringEncodingUTF8,
    //     //     )
    //     // };
    //     if unsafe {
    //         CFDictionaryGetValueIfPresent(a, window::kCGWindowOwnerName.to_void(), &mut value)
    //     } == 1
    //     {
    //         let strang = unsafe {
    //             CFStringGetCStringPtr(
    //                 value as CFStringRef,
    //                 core_foundation::string::kCFStringEncodingUTF8,
    //             )
    //         };

    //         if !strang.is_null() {
    //             dbg!(unsafe { CStr::from_ptr(strang) });
    //         }
    //         // dbg!(unsafe { *(value as CFStringRef) });
    //     };
    //     // dbg!(unsafe { a.offset(window::kCGWindowOwnerName) });
    //     // dbg!(*item);
    //     // let a = unsafe { std::mem::transmute::<_, CFDictionary<CFString, CFTypeRef>>(item) };
    //     // let m = unsafe { a.get(window::kCGWindowOwnerName) };

    //     // dbg!(unsafe { dbg!(*m) });
    //     dbg!(count);
    //     count += 1;
    // }

    Ok(())
}
