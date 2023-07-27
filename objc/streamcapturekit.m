#include <objc/runtime.h>
#include <ScreenCaptureKit/ScreenCaptureKit.h>

// Define two globals that reference each protocol, to make sure they are
// initialized in the Objective-C runtime.

id stream_output() {
    return @protocol(SCStreamOutput);
}

id stream_delegate() {
    return  @protocol(SCStreamDelegate);
}
