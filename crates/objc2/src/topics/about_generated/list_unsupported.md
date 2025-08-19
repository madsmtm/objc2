| Framework | Why is this unsupported? |
| --- | --- |
| `Accelerate` | Very C-centric, hard for us to map. |
| `ActivityKit` | Swift-only. |
| `AdAttributionKit` | Swift-only. |
| `AddressBook` | Deprecated, use Contacts instead. |
| `AddressBookUI` | Deprecated, use Contacts instead. |
| `AlarmKit` | Mostly Swift-only. |
| `AppIntents` | Swift-only. |
| `AppleScriptKit` | Basically empty nowadays. |
| `AppleScriptObjC` | Basically empty nowadays. |
| `AssetsLibrary` | Deprecated, use PhotoKit instead. |
| `Assignables` | Swift-only. |
| `AudioUnit` | Deprecated, use AudioToolbox instead. |
| `AudioVideoBridging` | Deprecated, use AVKit/AVFoundation instead (maybe?). |
| `AutomatedDeviceEnrollment` | Swift-only. |
| `BrowserKit` | TODO. |
| `CalendarStore` | Very deprecated. |
| `CarKey` | Swift-only. |
| `Charts` | Swift-only. |
| `Cocoa` | Basically empty nowadays, use AppKit. |
| `Combine` | Swift-only. |
| `ContactProvider` | Mostly Swift-only. |
| `CoreDisplay` | Basically empty. |
| `CoreHID` | Swift-only. |
| `CoreMIDIServer` | Very deprecated. |
| `CoreTransferable` | Swift-only. |
| `CreateML` | Swift-only. |
| `CreateMLComponents` | Swift-only. |
| `CryptoKit` | Swift-only. |
| `DVDPlayback` | Deprecated, use AVKit/AVFoundation instead. |
| `DeclaredAgeRange` | Swift-only. |
| `DeveloperToolsSupport` | Swift-only. |
| `DeviceActivity` | Swift-only. |
| `DeviceDiscoveryUI` | Needs Network first. |
| `DirectoryService` | Deprecated, use OpenDirectory instead. |
| `DiscRecording` | Deprecated, use AVKit/AVFoundation instead. |
| `DiscRecordingUI` | Deprecated, use AVKit/AVFoundation instead. |
| `DockKit` | Swift-only. |
| `DriverKit` | Uses C++ classes. |
| `EnergyKit` | Swift-only. |
| `ExtensionFoundation` | Swift-only. |
| `FamilyControls` | Swift-only. |
| `FinanceKit` | Swift-only. |
| `FinanceKitUI` | Swift-only. |
| `ForceFeedback` | Very C-centric and old. |
| `FoundationModels` | Swift-only. |
| `GLKit` | OpenGL-specific, use Metal instead. |
| `GLUT` | OpenGL-specific, use Metal instead. |
| `GSS` | Very C-centric and old. |
| `GameSave` | TODO. |
| `GeoToolbox` | Swift-only. |
| `GroupActivities` | Swift-only. |
| `Hypervisor` | Very low-level, consider crates like `applevisor` instead. |
| `ICADevices` | Deprecated, use ImageCaptureCore instead. |
| `IdentityDocumentServices` | Swift-only. |
| `IdentityDocumentServicesUI` | Swift-only. |
| `ImagePlayground` | Swift-only. |
| `ImmersiveMediaSupport` | Swift-only. |
| `InstallerPlugins` | Deprecated. |
| `InstantMessage` | Deprecated in macOS 10.9. |
| `InterfaceBuilderKit` | TODO. Developer-only. |
| `JavaNativeFoundation` | Basically empty. |
| `JavaRuntimeSupport` | Probably not really interesting.. |
| `JournalingSuggestions` | Swift-only. |
| `Kerberos` | Too low-level. |
| `Kernel` | Too low-level. |
| `KernelManagement` | Basically empty. |
| `LDAP` | Basically empty. |
| `LightweightCodeRequirements` | Swift-only. |
| `LiveCommunicationKit` | Swift-only. |
| `LockedCameraCapture` | Swift-only. |
| `ManagedApp` | Swift-only. |
| `ManagedAppDistribution` | Swift-only. |
| `ManagedSettings` | Swift-only. |
| `ManagedSettingsUI` | Swift-only. |
| `MarketplaceKit` | Swift-only. |
| `Matter` | Mostly available [here](https://github.com/project-chip/connectedhomeip). |
| `MatterSupport` | Swift-only. |
| `MediaLibrary` | Deprecated, use PhotoKit instead. |
| `Message` | Basically empty. |
| `MetalPerformancePrimitives` | Header-only framework used in Metal shaders. |
| `MobileCoreServices` | Deprecated, use CoreServices + UniformTypeIdentifiers instead. |
| `MusicKit` | Swift-only. |
| `NetFS` | Deprecated, use macFUSE or FSKit instead (probably). |
| `Network` | TODO, see [#646](https://github.com/madsmtm/objc2/issues/646). |
| `OpenAL` | Very C-centric, use newer Audio frameworks instead. |
| `OpenCL` | Very C-centric and old. |
| `OpenGL` | OpenGL-specific, use Metal instead. |
| `OpenGLES` | OpenGL-specific, use Metal instead. |
| `PCSC` | Too low-level, consider crates like `pcsc` instead. |
| `PaperKit` | Swift-only. |
| `PermissionKit` | Swift-only. |
| `ProximityReader` | Swift-only. |
| `ProximityReaderStub` | Basically empty. |
| `Python3` | Better served by dedicated crates like `pyo3-ffi`. |
| `QTKit` | No headers present in Xcode's SDK. |
| `RealityFoundation` | Swift-only. |
| `RealityKit` | Swift-only. |
| `RelevanceKit` | Swift-only. |
| `RoomPlan` | Swift-only. |
| `Ruby` | Very C-centric and old. |
| `SecureElementCredential` | Swift-only. |
| `SecurityUI` | TODO. |
| `StickerFoundation` | Basically empty. |
| `StickerKit` | Basically empty. |
| `StoreKitTest` | TODO. Developer-only. |
| `SwiftData` | Swift-only. |
| `SwiftUI` | Swift-only. |
| `SwiftUICore` | Swift-only. |
| `SyncServices` | Deprecated. |
| `System` | Deprecated wrapper over libSystem.dylib. |
| `TWAIN` | Very C-centric and old. |
| `TabletopKit` | Swift-only. |
| `TabularData` | Swift-only. |
| `Tcl` | Very C-centric and old. |
| `TelephonyMessagingKit` | Swift-only. |
| `Testing` | Swift-only. |
| `TipKit` | Swift-only. |
| `Tk` | Very C-centric and old. |
| `TouchController` | TODO. |
| `Translation` | Swift-only. |
| `TranslationUIProvider` | Swift-only. |
| `Twitter` | Deprecated, use Social instead. |
| `VideoDecodeAcceleration` | Very C-centric and old. |
| `VisionEntitlementServices` | Mostly Swift-only. |
| `VisionKit` | Swift-only. |
| `VisualIntelligence` | Swift-only. |
| `WeatherKit` | Swift-only. |
| `WiFiAware` | Swift-only. |
| `WidgetKit` | Mostly Swift-only. |
| `WirelessInsights` | Swift-only. |
| `WorkoutKit` | Swift-only. |
| `XcodeKit` | TODO. Developer-only. |
| `iAd` | Disabled on server side, use AdServices instead. |
| `vecLib` | Very C-centric and old. |
| `vmnet` | Very C-centric and old. |
