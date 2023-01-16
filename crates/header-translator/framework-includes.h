// Workaround for clang < 13, only used in NSBundle.h
#define NS_FORMAT_ARGUMENT(A)

// Workaround for clang < 13
#define _Nullable_result _Nullable

#include <TargetConditionals.h>

#import <Foundation/Foundation.h>

#import <QuartzCore/CoreAnimation.h>

#import <CoreData/CoreData.h>

#if TARGET_OS_OSX
#import <AppKit/AppKit.h>
#endif

#import <AuthenticationServices/AuthenticationServices.h>

#import <Metal/Metal.h>

#import <WebKit/WebKit.h>
