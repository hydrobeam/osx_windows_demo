#![allow(unused_imports)]
use block2::{Block, ConcreteBlock};
use core_foundation::base::ToVoid;
use core_foundation::dictionary::{CFDictionary, CFDictionaryGetValueIfPresent, CFDictionaryRef};
use core_foundation::string::{kCFStringEncodingUTF8, CFString, CFStringGetCStringPtr};
use core_foundation::{base::CFTypeRef, string::CFStringRef};
use core_graphics::{event, window};
use dispatch::ffi::{
    dispatch_get_main_queue, dispatch_object_s, dispatch_queue_attr_t, dispatch_queue_create,
    dispatch_queue_t,
};
use icrate::ns_string;
use objc2::rc::{Allocated, Id};
use std::ffi::{c_void, CStr, CString};
use std::ops::Deref;
pub type CMTimeValue = i64;
pub type CMTimeScale = i32;
pub type CMTimeEpoch = i64;
pub const CMTimeFlags_kCMTimeFlags_Valid: CMTimeFlags = 1;
pub const CMTimeFlags_kCMTimeFlags_HasBeenRounded: CMTimeFlags = 2;
pub const CMTimeFlags_kCMTimeFlags_PositiveInfinity: CMTimeFlags = 4;
pub const CMTimeFlags_kCMTimeFlags_NegativeInfinity: CMTimeFlags = 8;
pub const CMTimeFlags_kCMTimeFlags_Indefinite: CMTimeFlags = 16;
pub const CMTimeFlags_kCMTimeFlags_ImpliedValueFlagsMask: CMTimeFlags = 28;
pub type CMTimeFlags = u32;
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct CMTime {
    pub value: CMTimeValue,
    pub timescale: CMTimeScale,
    pub flags: CMTimeFlags,
    pub epoch: CMTimeEpoch,
}

unsafe impl Encode for CMTime {
    const ENCODING: Encoding = Encoding::Struct(
        "CMTime",
        &[
            Encoding::LongLong,
            Encoding::Int,
            Encoding::LongLong,
            Encoding::UInt,
        ],
    );
}

unsafe impl RefEncode for CMTime {
    const ENCODING_REF: Encoding = Self::ENCODING;
}

// unsafe impl Encode for CGRect {
//     const ENCODING: Encoding =
//         Encoding::Struct(names::RECT, &[CGPoint::ENCODING, CGSize::ENCODING]);
// }
use dispatch::Queue;

use objc2::{
    class, extern_class, msg_send, mutability,
    runtime::{Class, Object},
    ClassType,
};

struct MyQueue(Queue);
impl Deref for MyQueue {
    type Target = Queue;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

unsafe impl Encode for MyQueue {
    const ENCODING: objc2::Encoding = objc2::Encoding::Object;
}

unsafe impl RefEncode for MyQueue {
    const ENCODING_REF: objc2::Encoding = objc2::Encoding::Object;
}

use icrate::Foundation::{CGRect, NSArray, NSErrorDomain, NSObject, NSObjectProtocol, NSString};

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
use objc2::{
    declare_class, extern_methods, extern_protocol, msg_send_id, Encode, Encoding, RefEncode,
};

use objc2::ProtocolType;

use icrate::Foundation::NSInteger;
extern_protocol!(
    /// This comment will appear on the trait as expected.
    pub unsafe trait SCStreamOutput: NSObjectProtocol {
        #[method(stream:didOutputSampleBuffer:ofType:)]
        fn stream(the_stream: *const NSObject, sample_buffer: *const NSObject, output_type: NSInteger);
    }
    unsafe impl ProtocolType for dyn SCStreamOutput {}
);

extern_protocol!(
    /// This comment will appear on the trait as expected.
    pub unsafe trait SCStreamDelegate: NSObjectProtocol {
        #[method(stream:didStopWithError:)]
        fn stream_delegate(stream: *const NSObject, did_stop_with_error: *const NSError);
    }
    unsafe impl ProtocolType for dyn SCStreamDelegate {}
);

declare_class!(
    #[derive(Debug)]
    struct StreamEat {}

    unsafe impl ClassType for StreamEat {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "StreamEat";
    }

    unsafe impl SCStreamOutput for StreamEat {
        #[method(stream:didOutputSampleBuffer:ofType:)]
        unsafe fn stream(stream: *const Object, didOutputSampleBuffer: &Object, ofType: NSInteger) {
            let a: *const i32 = std::ptr::null();
            unsafe { *a };
        }
    }

    unsafe impl SCStreamDelegate for StreamEat {
        #[method(stream:didStopWithError:)]
        unsafe fn stream_delegate(stream: *const Object, did_stop_with_error: *const NSError) {
            let a: *const i32 = std::ptr::null();
            unsafe { *a };
        }
    }
);
unsafe impl NSObjectProtocol for StreamEat {}

#[derive(Debug)]
struct SendPtr<T>(*const T);

unsafe impl<T> Encode for SendPtr<T> {
    const ENCODING: objc2::Encoding = objc2::Encoding::Object;
}

#[link(name = "ScreenCaptureKit", kind = "framework")]
extern "C" {}
#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}
#[link(name = "CoreMedia", kind = "framework")]
extern "C" {
    pub fn CMTimeMake(value: i64, timescale: i32) -> CMTime;
}
#[link(name = "AVFoundation", kind = "framework")]
extern "C" {}

fn main() -> Result<(), ()> {
    let sc_content_filter = class!(SCContentFilter);
    let sc_stream_configuration = class!(SCStreamConfiguration);
    let sc_stream = class!(SCStream);
    let block = ConcreteBlock::new(
        |shareable_content: *const SCShareableContent, error: *const NSError| {
            if !error.is_null() {
                panic!("unable to fetch windows, make sure permissions are granted");
            }
            let displays: &NSArray = unsafe { msg_send![shareable_content, displays] };
            for display in displays.iter() {
                {
                    // SCWindow
                    let cg_rect: CGRect = unsafe { msg_send![display, frame] };
                    let h = cg_rect.size.height as u64;
                    let w = cg_rect.size.width as u64;
                    let f_obj = unsafe { msg_send_id![sc_content_filter, alloc] };

                    let null = NSArray::<Object>::new();
                    let filter: Id<NSObject> = unsafe {
                        msg_send_id![f_obj, initWithDisplay:display
                        excludingWindows:&*null]
                    };

                    let stream_config: Id<NSObject> =
                        unsafe { msg_send_id![msg_send_id![sc_stream_configuration, alloc], init] };

                    unsafe {
                        let _: () = msg_send![&*stream_config, setWidth:w];
                        let _: () = msg_send![&*stream_config, setHeight:h];
                        let _: () = msg_send![&*stream_config, setQueueDepth:6_i64];
                    };

                    let stream_output_consumer: Id<StreamEat> =
                        unsafe { msg_send_id![StreamEat::alloc(), init] };
                    dbg!(&***stream_output_consumer);

                    let stream: Id<NSObject> = unsafe {
                        msg_send_id![
                            msg_send_id![sc_stream, alloc], initWithFilter:&*filter
                            configuration:&*stream_config
                            delegate:&*stream_output_consumer
                        ]
                    };
                    let err = NSError::new(44, ns_string!("ScreenRecorder.WackyError"));
                    let label = CString::new("ScreenRecorder.VideoSampleBufferQueue").unwrap();
                    let attr = 0 as dispatch_queue_attr_t;
                    let queue = SendPtr(unsafe { dispatch_queue_create(label.as_ptr(), attr) });
                    let queue: *const Object = std::ptr::null();
                    // let queue = SendPtr(dispatch_get_main_queue());
                    let did_setup: bool = unsafe {
                        msg_send![&stream,
                                  addStreamOutput:&*stream_output_consumer
                                  type:0_i64
                                  sampleHandlerQueue:queue
                                  error:&&*err
                        ]
                    };
                    dbg!(did_setup);
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
                    break;
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
    std::thread::sleep(std::time::Duration::from_secs(10));

    // unsafe { msg_send![qq, completionHandler:&block] }

    // block::Block

    // let m = unsafe { msg_send![available_content, excludingDesktopWindows, false,] };
    // let windows = window::copy_window_info(
    // window::kCGWindowListOptionAll
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
