#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use block2::ConcreteBlock;
use icrate::{
    ns_string,
    Foundation::{NSArray, NSError, NSInteger, NSObject, NSObjectProtocol},
};

use objc2::{
    class, declare_class, extern_class, extern_protocol, msg_send, msg_send_id, mutability,
    runtime::Object, ClassType, Encode, Encoding, ProtocolType, RefEncode,
};
use objc2::{rc::Id, runtime};

extern_protocol!(
    pub unsafe trait SCStreamOutput: NSObjectProtocol {
        #[method(stream:didOutputSampleBuffer:ofType:)]
        fn stream(
            &self,
            the_stream: *const Object,
            sample_buffer: *mut Object,
            output_type: NSInteger,
        );
    }
    unsafe impl ProtocolType for dyn SCStreamOutput {}
);

extern_protocol!(
    pub unsafe trait SCStreamDelegate: NSObjectProtocol {
        #[method(stream:didStopWithError:)]
        fn stream_delegate(&self, stream: *const Object, did_stop_with_error: *const NSError);
    }
    unsafe impl ProtocolType for dyn SCStreamDelegate {}
);

declare_class!(
    #[derive(Debug)]
    struct StreamEat {}

    unsafe impl ClassType for StreamEat {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "osx_StreamEat";
    }

    unsafe impl SCStreamOutput for StreamEat {
        #[method(stream:didOutputSampleBuffer:ofType:)]
        unsafe fn stream(
            &self,
            _stream: *const Object,
            _sampleBuffer: *mut Object,
            _ofType: NSInteger,
        ) {
            dbg!("OUTPUT");
        }
    }

    unsafe impl SCStreamDelegate for StreamEat {
        #[method(stream:didStopWithError:)]
        unsafe fn stream_delegate(
            &self,
            _stream: *const Object,
            _did_stop_with_error: *const NSError,
        ) {
            dbg!("DELEGATE");
        }
    }

    unsafe impl NSObjectProtocol for StreamEat {}
);

#[derive(Debug)]
struct SendPtr<T>(*const T);

unsafe impl<T> Encode for SendPtr<T> {
    const ENCODING: objc2::Encoding = objc2::Encoding::Object;
}

#[link(name = "ScreenCaptureKit", kind = "framework")]
extern "C" {}
#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}
#[link(name = "AVFoundation", kind = "framework")]
extern "C" {}

extern "C" {
    fn stream_output() -> *const Object;
    fn stream_delegate() -> *const Object;
}

fn main() -> Result<(), ()> {
    let sc_content_filter = class!(SCContentFilter);
    let sc_stream_configuration = class!(SCStreamConfiguration);
    let sc_stream = class!(SCStream);
    dbg!(runtime::Protocol::get("SCStreamDelegate"));
    dbg!(runtime::Protocol::get("SCStreamOutput"));
    unsafe {
        dbg!(stream_delegate(), stream_output());
    }
    // this is handled after the next call, see end of main
    let block = ConcreteBlock::new(|shareable_content: *const Object, error: *const NSError| {
        if !error.is_null() {
            panic!("unable to fetch windows, make sure permissions are granted");
        }
        let displays: &NSArray = unsafe { msg_send![shareable_content, displays] };

        if let Some(display) = displays.iter().next() {
            let f_obj = unsafe { msg_send_id![sc_content_filter, alloc] };

            let null = NSArray::<Object>::new();
            let filter: Id<NSObject> = unsafe {
                msg_send_id![f_obj, initWithDisplay:display
                        excludingWindows:&*null]
            };

            let stream_config: Id<NSObject> =
                unsafe { msg_send_id![msg_send_id![sc_stream_configuration, alloc], init] };

            let stream_output_consumer: Id<StreamEat> =
                unsafe { msg_send_id![StreamEat::alloc(), init] };

            // this successfully triggers the message

            // let null_obj: *const Object = std::ptr::null();
            // let _: () = unsafe {
            //     msg_send![&*stream_output_consumer, stream:null_obj didOutputSampleBuffer:null_obj ofType:1_i64]
            // };

            let stream: Id<NSObject> = unsafe {
                msg_send_id![
                    msg_send_id![sc_stream, alloc], initWithFilter:&*filter
                    configuration:&*stream_config
                    delegate:&*stream_output_consumer
                ]
            };
            let err = NSError::new(44, ns_string!("ScreenRecorder.WackyError"));

            // queue shenanigans

            // let a = std::ptr::null() as *const *const Object;
            // let label = CString::new("ScreenRecorder.VideoSampleBufferQueue").unwrap();
            // let attr = 0 as dispatch_queue_attr_t;
            // let queue = SendPtr(unsafe { dispatch_queue_create(label.as_ptr(), attr) });
            let queue = SendPtr(dispatch::ffi::dispatch_get_main_queue());

            // let queue: *const Object = std::ptr::null();
            let did_setup: bool = unsafe {
                msg_send![&stream,
                          addStreamOutput:&*stream_output_consumer
                          type:0_i64
                          sampleHandlerQueue:queue
                          error:&&*err
                ]
            };
            dbg!(did_setup);

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
    });

    let sc_shareable = class!(SCShareableContent);
    unsafe {
        let _: () = msg_send![
            sc_shareable,
            getShareableContentWithCompletionHandler:&block
        ];
    };
    // give the callback time to execute
    std::thread::sleep(std::time::Duration::from_secs(10));

    Ok(())
}
