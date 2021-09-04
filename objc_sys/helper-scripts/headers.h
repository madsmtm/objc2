// Generate bindings for Objective-C version 2
#define __OBJC2__ 1

// Make the objc_msgSend family not variadic
//
// See https://developer.apple.com/documentation/objectivec/objc_old_dispatch_prototypes?language=objc
#define OBJC_OLD_DISPATCH_PROTOTYPES 0


// Public headers
// The order of these is important

#include "objc-api.h"
#include "objc.h"
#include "runtime.h"
#include "objc-exception.h"
#include "objc-sync.h"

// All of these are custom-defined in message.rs because of complex cfgs
// #include "message.h"

// Not used, contains old GC stuff
// Could also be removed with #define OBJC_NO_GC_API 1
// #include "objc-auto.h"


// Private headers

// Contains debug objc_msgSend + some Property stuff
// #include "objc-abi.h"

// Contains internals + ARC
// #include "objc-internal.h"
