use block2::{Block, ConcreteBlock};
use core_foundation::base::ToVoid;
use core_foundation::dictionary::{CFDictionary, CFDictionaryGetValueIfPresent, CFDictionaryRef};
use core_foundation::string::{kCFStringEncodingUTF8, CFString, CFStringGetCStringPtr};
use core_foundation::{base::CFTypeRef, string::CFStringRef};
use core_graphics::{event, window};
use objc2::rc::{Allocated, Id};
use std::ffi::{c_void, CStr};

use objc2::{
    class, extern_class, msg_send, mutability,
    runtime::{Class, Object},
    ClassType,
};

use icrate::Foundation::{NSArray, NSObject, NSObjectProtocol, NSString};

extern_class!(
    #[derive(PartialEq, Eq, Hash)] // Uses the superclass' implementation
    pub struct SCShareableContent;
    unsafe impl ClassType for SCShareableContent {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

extern_class!(
    #[derive(PartialEq, Eq, Hash)] // Uses the superclass' implementation
    pub struct CMSampleBuffer;
    unsafe impl ClassType for CMSampleBuffer {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
    }
);

use icrate::Foundation::NSError;
use objc2::{declare_class, extern_methods, extern_protocol, msg_send_id, Encode, RefEncode};

use objc2::ProtocolType;
// //                                  ^^^^^^^^^^^^^^^^
// // This protocol inherits from the `NSObject` protocol

// // This method we mark as `unsafe`, since we aren't using the correct
// // type for the completion handler
// #[method_id(loadDataWithTypeIdentifier:forItemProviderCompletionHandler:)]
// unsafe fn loadData(
//     &self,
//     type_identifier: &NSString,
//     completion_handler: *mut c_void,
// ) -> Option<Id<NSProgress>>;

// #[method_id(writableTypeIdentifiersForItemProvider)]
// fn writableTypeIdentifiersForItemProvider_class()
//     -> Id<NSArray<NSString>>;

// // The rest of these are optional, which means that a user of
// // `declare_class!` would not need to implement them.

// #[optional]
// #[method_id(writableTypeIdentifiersForItemProvider)]
// fn writableTypeIdentifiersForItemProvider(&self)
//     -> Id<NSArray<NSString>>;

// #[optional]
// #[method(itemProviderVisibilityForRepresentationWithTypeIdentifier:)]
// fn itemProviderVisibilityForRepresentation_class(
//     type_identifier: &NSString,
// ) -> NSItemProviderRepresentationVisibility;

// #[optional]
// #[method(itemProviderVisibilityForRepresentationWithTypeIdentifier:)]
// fn itemProviderVisibilityForRepresentation(
//     &self,
//     type_identifier: &NSString,
// ) -> NSItemProviderRepresentationVisibility;

use icrate::Foundation::NSInteger;
extern_protocol!(
    /// This comment will appear on the trait as expected.
    pub unsafe trait StreamOutput: NSObjectProtocol {
        #[method(stream:didOutputSampleBuffer:ofType:)]
        fn stream(
            the_stream: *const Object,
            sample_buffer: *const Object,
            output_type: NSInteger,
        ) -> ();
    }
    unsafe impl ProtocolType for dyn StreamOutput {}
);

use objc2::declare::IvarEncode;
declare_class!(
    struct StreamEat {
        // foo: IvarEncode<u8, "_foo">,
        // pub bar: IvarEncode<c_int, "_bar">,
        // object: IvarDrop<Id<NSObject>, "_object">,
    }

    unsafe impl ClassType for StreamEat {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "StreamEat";
    }
    unsafe impl NSObjectProtocol for StreamEat {}

    unsafe impl StreamOutput for StreamEat {
        #[method(stream)]
        unsafe fn stream(
            the_stream: *const Object,
            sample_buffer: *const Object,
            output_type: NSInteger,
        ) {
            dbg!("hi");
        }
    }
);

extern_protocol!(
    /// This comment will appear on the trait as expected.
    pub unsafe trait SCStreamDelegate: NSObjectProtocol {
        #[method(stream:didStopWithError:)]
        fn stream(stream: *const Object, did_stop_with_error: *const NSError) -> ();
    }
    unsafe impl ProtocolType for dyn SCStreamDelegate {}
);

declare_class!(
    struct SCDelegate {}

    unsafe impl ClassType for SCDelegate {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "SCDelegate";
    }
    unsafe impl NSObjectProtocol for SCDelegate {}

    unsafe impl SCStreamDelegate for SCDelegate {
        #[method(stream)]
        unsafe fn stream(stream: *const Object, did_stop_with_error: *const NSError) {
            dbg!("hi");
        }
    }
);
unsafe impl Encode for StreamEat {
    const ENCODING: objc2::Encoding = objc2::Encoding::Object;
}

unsafe impl RefEncode for dyn StreamOutput {
    const ENCODING_REF: objc2::Encoding = objc2::Encoding::Object;
}
// #[no_mangle]
// pub unsafe extern "C" fn wlist_basic(block: &ConcreteBlock<(i32, i32), i32>) -> i32 {
//     block.call((5, 8))
// }

// Required to bring NSPasteboard into the path of the class-resolver
#[link(name = "ScreenCaptureKit", kind = "framework")]
extern "C" {}

fn main() -> Result<(), ()> {
    let sc_content_filter = class!(SCContentFilter);
    let sc_stream_configuration = class!(SCStreamConfiguration);
    let sc_stream = class!(SCStream);
    let block = ConcreteBlock::new(
        |shareable_content: *const SCShareableContent, error: *const NSError| {
            if !error.is_null() {
                panic!("unable to fetch windows, make sure permissions are granted")
            }

            // array of SCWindows
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
                if title.contains("osx") {
                    // SCWindow
                    let f_obj = unsafe { msg_send_id![sc_content_filter, alloc] };
                    let filter: Id<NSObject> =
                        unsafe { msg_send_id![f_obj, initWithDesktopIndependentWindow:window] };

                    let stream_config: Id<NSObject> =
                        unsafe { msg_send_id![msg_send_id![sc_stream_configuration, alloc], init] };

                    let stream_output_consumer: Id<StreamEat> =
                        unsafe { msg_send_id![StreamEat::alloc(), init] };

                    let delegate: Id<SCDelegate> =
                        unsafe { msg_send_id![SCDelegate::alloc(), init] };

                    let stream: Id<NSObject> = unsafe {
                        msg_send_id![
                            msg_send_id![sc_stream, alloc], initWithFilter:&*filter
                            configuration:&*stream_config
                            delegate:&*delegate
                        ]
                    };

                    let stream_handler = ConcreteBlock::new(|error: *const NSError| {
                        if !error.is_null() {
                            panic!("unable to initialize stream")
                        }
                    });
                    // msg_send![stream, ]

                    // StreamOutput::stream(the_stream, sample_buffer, output_type)

                    // let stream_output_consumer =
                    //     unsafe { std::mem::transmute::<_, Id<NSObject>>(stream_output_consumer) };

                    let did_setup: bool = unsafe {
                        msg_send![&stream, addStreamOutput:&*stream_output_consumer type:1 ]
                    };
                    // let meow = eater.into();
                    let basic_completion_handler = ConcreteBlock::new(|error: *const NSError| {
                        if !error.is_null() {
                            panic!("something went wrong with starting the stream capture")
                        } else {
                            println!("Started streaming!!!!!")
                        }
                    });
                    let _: () = unsafe {
                        msg_send![&stream, startCaptureWithCompletionHandler:&basic_completion_handler]
                    };
                }
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
