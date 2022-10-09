use super::__exported::NSFont;
use super::__exported::NSFontDescriptor;
use super::__exported::NSFontPanel;
use super::__exported::NSMenu;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSMenu::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSDictionary::*;
use crate::Foundation::generated::NSGeometry::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSFontManager;
    unsafe impl ClassType for NSFontManager {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSFontManager {
        #[method(setFontPanelFactory:)]
        pub unsafe fn setFontPanelFactory(factoryId: Option<&Class>);
        #[method(setFontManagerFactory:)]
        pub unsafe fn setFontManagerFactory(factoryId: Option<&Class>);
        #[method_id(sharedFontManager)]
        pub unsafe fn sharedFontManager() -> Id<NSFontManager, Shared>;
        #[method(isMultiple)]
        pub unsafe fn isMultiple(&self) -> bool;
        #[method_id(selectedFont)]
        pub unsafe fn selectedFont(&self) -> Option<Id<NSFont, Shared>>;
        #[method(setSelectedFont:isMultiple:)]
        pub unsafe fn setSelectedFont_isMultiple(&self, fontObj: &NSFont, flag: bool);
        #[method(setFontMenu:)]
        pub unsafe fn setFontMenu(&self, newMenu: &NSMenu);
        #[method_id(fontMenu:)]
        pub unsafe fn fontMenu(&self, create: bool) -> Option<Id<NSMenu, Shared>>;
        #[method_id(fontPanel:)]
        pub unsafe fn fontPanel(&self, create: bool) -> Option<Id<NSFontPanel, Shared>>;
        #[method_id(fontWithFamily:traits:weight:size:)]
        pub unsafe fn fontWithFamily_traits_weight_size(
            &self,
            family: &NSString,
            traits: NSFontTraitMask,
            weight: NSInteger,
            size: CGFloat,
        ) -> Option<Id<NSFont, Shared>>;
        #[method(traitsOfFont:)]
        pub unsafe fn traitsOfFont(&self, fontObj: &NSFont) -> NSFontTraitMask;
        #[method(weightOfFont:)]
        pub unsafe fn weightOfFont(&self, fontObj: &NSFont) -> NSInteger;
        #[method_id(availableFonts)]
        pub unsafe fn availableFonts(&self) -> Id<NSArray<NSString>, Shared>;
        #[method_id(availableFontFamilies)]
        pub unsafe fn availableFontFamilies(&self) -> Id<NSArray<NSString>, Shared>;
        #[method_id(availableMembersOfFontFamily:)]
        pub unsafe fn availableMembersOfFontFamily(
            &self,
            fam: &NSString,
        ) -> Option<Id<NSArray<NSArray>, Shared>>;
        #[method_id(convertFont:)]
        pub unsafe fn convertFont(&self, fontObj: &NSFont) -> Id<NSFont, Shared>;
        #[method_id(convertFont:toSize:)]
        pub unsafe fn convertFont_toSize(
            &self,
            fontObj: &NSFont,
            size: CGFloat,
        ) -> Id<NSFont, Shared>;
        #[method_id(convertFont:toFace:)]
        pub unsafe fn convertFont_toFace(
            &self,
            fontObj: &NSFont,
            typeface: &NSString,
        ) -> Option<Id<NSFont, Shared>>;
        #[method_id(convertFont:toFamily:)]
        pub unsafe fn convertFont_toFamily(
            &self,
            fontObj: &NSFont,
            family: &NSString,
        ) -> Id<NSFont, Shared>;
        #[method_id(convertFont:toHaveTrait:)]
        pub unsafe fn convertFont_toHaveTrait(
            &self,
            fontObj: &NSFont,
            trait_: NSFontTraitMask,
        ) -> Id<NSFont, Shared>;
        #[method_id(convertFont:toNotHaveTrait:)]
        pub unsafe fn convertFont_toNotHaveTrait(
            &self,
            fontObj: &NSFont,
            trait_: NSFontTraitMask,
        ) -> Id<NSFont, Shared>;
        #[method_id(convertWeight:ofFont:)]
        pub unsafe fn convertWeight_ofFont(
            &self,
            upFlag: bool,
            fontObj: &NSFont,
        ) -> Id<NSFont, Shared>;
        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;
        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);
        #[method(action)]
        pub unsafe fn action(&self) -> Sel;
        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Sel);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<Object, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&Object>);
        #[method(sendAction)]
        pub unsafe fn sendAction(&self) -> bool;
        #[method_id(localizedNameForFamily:face:)]
        pub unsafe fn localizedNameForFamily_face(
            &self,
            family: &NSString,
            faceKey: Option<&NSString>,
        ) -> Id<NSString, Shared>;
        #[method(setSelectedAttributes:isMultiple:)]
        pub unsafe fn setSelectedAttributes_isMultiple(
            &self,
            attributes: &NSDictionary<NSString, Object>,
            flag: bool,
        );
        #[method_id(convertAttributes:)]
        pub unsafe fn convertAttributes(
            &self,
            attributes: &NSDictionary<NSString, Object>,
        ) -> Id<NSDictionary<NSString, Object>, Shared>;
        #[method_id(availableFontNamesMatchingFontDescriptor:)]
        pub unsafe fn availableFontNamesMatchingFontDescriptor(
            &self,
            descriptor: &NSFontDescriptor,
        ) -> Option<Id<NSArray, Shared>>;
        #[method_id(collectionNames)]
        pub unsafe fn collectionNames(&self) -> Id<NSArray, Shared>;
        #[method_id(fontDescriptorsInCollection:)]
        pub unsafe fn fontDescriptorsInCollection(
            &self,
            collectionNames: &NSString,
        ) -> Option<Id<NSArray, Shared>>;
        #[method(addCollection:options:)]
        pub unsafe fn addCollection_options(
            &self,
            collectionName: &NSString,
            collectionOptions: NSFontCollectionOptions,
        ) -> bool;
        #[method(removeCollection:)]
        pub unsafe fn removeCollection(&self, collectionName: &NSString) -> bool;
        #[method(addFontDescriptors:toCollection:)]
        pub unsafe fn addFontDescriptors_toCollection(
            &self,
            descriptors: &NSArray,
            collectionName: &NSString,
        );
        #[method(removeFontDescriptor:fromCollection:)]
        pub unsafe fn removeFontDescriptor_fromCollection(
            &self,
            descriptor: &NSFontDescriptor,
            collection: &NSString,
        );
        #[method(currentFontAction)]
        pub unsafe fn currentFontAction(&self) -> NSFontAction;
        #[method(convertFontTraits:)]
        pub unsafe fn convertFontTraits(&self, traits: NSFontTraitMask) -> NSFontTraitMask;
        #[method_id(target)]
        pub unsafe fn target(&self) -> Option<Id<Object, Shared>>;
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);
    }
);
extern_methods!(
    #[doc = "NSFontManagerMenuActionMethods"]
    unsafe impl NSFontManager {
        #[method(fontNamed:hasTraits:)]
        pub unsafe fn fontNamed_hasTraits(
            &self,
            fName: &NSString,
            someTraits: NSFontTraitMask,
        ) -> bool;
        #[method_id(availableFontNamesWithTraits:)]
        pub unsafe fn availableFontNamesWithTraits(
            &self,
            someTraits: NSFontTraitMask,
        ) -> Option<Id<NSArray<NSString>, Shared>>;
        #[method(addFontTrait:)]
        pub unsafe fn addFontTrait(&self, sender: Option<&Object>);
        #[method(removeFontTrait:)]
        pub unsafe fn removeFontTrait(&self, sender: Option<&Object>);
        #[method(modifyFontViaPanel:)]
        pub unsafe fn modifyFontViaPanel(&self, sender: Option<&Object>);
        #[method(modifyFont:)]
        pub unsafe fn modifyFont(&self, sender: Option<&Object>);
        #[method(orderFrontFontPanel:)]
        pub unsafe fn orderFrontFontPanel(&self, sender: Option<&Object>);
        #[method(orderFrontStylesPanel:)]
        pub unsafe fn orderFrontStylesPanel(&self, sender: Option<&Object>);
    }
);
extern_methods!(
    #[doc = "NSFontManagerDelegate"]
    unsafe impl NSObject {
        #[method(fontManager:willIncludeFont:)]
        pub unsafe fn fontManager_willIncludeFont(
            &self,
            sender: &Object,
            fontName: &NSString,
        ) -> bool;
    }
);
extern_methods!(
    #[doc = "NSFontManagerResponderMethod"]
    unsafe impl NSObject {
        #[method(changeFont:)]
        pub unsafe fn changeFont(&self, sender: Option<&Object>);
    }
);
