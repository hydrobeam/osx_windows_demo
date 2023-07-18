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
    // SCShareableContent.
    // core_graphics::display::CGWindowListCopyWindowInfo(option, relativeToWindow)
    // let a: *mut Object = ;
    // let available_content: *mut Object = unsafe { msg_send![SCShareableContent, new] };
    // available_content;
    // SCShareableContent:
    let SCWindow = class!(SCWindow);
    // let CGWindowID = class!(CGWindowID);
    let block = ConcreteBlock::new(
        |shareableContent: *const SCShareableContent, error: *const NSError| {
            // if error.is_
            dbg!(shareableContent);
            dbg!(error);
            let windows: *const NSArray = unsafe { msg_send![shareableContent, applications] };
            dbg!(windows);
            unsafe {
                for window in (*windows).iter() {
                    dbg!(window);
                    let ret: *const NSString = msg_send![window, title];
                    let utf8title = (*ret).UTF8String();
                    let title = CStr::from_ptr(utf8title);
                    dbg!(title);
                    // std::str::from_utf8(utf8title);
                }
            }
            ();
        },
    );
    // let block= block.clone_into()
    // let block = &block.copy();

    // block
    let sc_shareable = class!(SCShareableContent);
    dbg!("??");
    unsafe {
        let a: () = msg_send![
            sc_shareable,
            getShareableContentWithCompletionHandler:&block
        ];
    };

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
