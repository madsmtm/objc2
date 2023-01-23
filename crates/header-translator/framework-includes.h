// Workaround for clang < 13, only used in NSBundle.h
#define NS_FORMAT_ARGUMENT(A)

// Workaround for clang < 13
#define _Nullable_result _Nullable

#include <TargetConditionals.h>

#if TARGET_OS_OSX
#import <AppKit/AppKit.h>
#import <Automator/Automator.h>
#import <OSAKit/OSAKit.h>
#endif

#import <Accessibility/Accessibility.h>

#import <AdServices/AdServices.h>

#import <AdSupport/AdSupport.h>

#import <AuthenticationServices/AuthenticationServices.h>

#import <AutomaticAssessmentConfiguration/AutomaticAssessmentConfiguration.h>

#import <BackgroundAssets/BackgroundAssets.h>

#import <BackgroundTasks/BackgroundTasks.h>

#import <ClassKit/ClassKit.h>

#import <CloudKit/CloudKit.h>

#import <Contacts/Contacts.h>

#import <CoreData/CoreData.h>

#import <CoreLocation/CoreLocation.h>

#import <EventKit/EventKit.h>

#import <ExceptionHandling/ExceptionHandling.h>

#import <Foundation/Foundation.h>

#import <GameKit/GameKit.h>

#import <InputMethodKit/InputMethodKit.h>

#import <MapKit/MapKit.h>

#import <Metal/Metal.h>

#import <MetalFX/MetalFX.h>

#import <MetalKit/MetalKit.h>

#import <QuartzCore/CoreAnimation.h>

#import <WebKit/WebKit.h>
