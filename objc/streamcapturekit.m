#include <objc/runtime.h>
#include <ScreenCaptureKit/ScreenCaptureKit.h>

// Define two globals that reference each protocol, to make sure they are
// initialized in the Objective-C runtime.
Protocol* SCStreamOutput_protocol_hack = @protocol(SCStreamOutput);
Protocol* SCStreamDelegate_protocol_hack = @protocol(SCStreamDelegate);
