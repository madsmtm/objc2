#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSMenuItem;
    unsafe impl ClassType for NSMenuItem {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSMenuItem {
        #[method(usesUserKeyEquivalents)]
        pub unsafe fn usesUserKeyEquivalents() -> bool;
        #[method(setUsesUserKeyEquivalents:)]
        pub unsafe fn setUsesUserKeyEquivalents(usesUserKeyEquivalents: bool);
        #[method_id(separatorItem)]
        pub unsafe fn separatorItem() -> Id<NSMenuItem, Shared>;
        #[method_id(initWithTitle:action:keyEquivalent:)]
        pub unsafe fn initWithTitle_action_keyEquivalent(
            &self,
            string: &NSString,
            selector: Option<Sel>,
            charCode: &NSString,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Id<Self, Shared>;
        #[method_id(menu)]
        pub unsafe fn menu(&self) -> Option<Id<NSMenu, Shared>>;
        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: Option<&NSMenu>);
        #[method(hasSubmenu)]
        pub unsafe fn hasSubmenu(&self) -> bool;
        #[method_id(submenu)]
        pub unsafe fn submenu(&self) -> Option<Id<NSMenu, Shared>>;
        #[method(setSubmenu:)]
        pub unsafe fn setSubmenu(&self, submenu: Option<&NSMenu>);
        #[method_id(parentItem)]
        pub unsafe fn parentItem(&self) -> Option<Id<NSMenuItem, Shared>>;
        #[method_id(title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);
        #[method_id(attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Option<Id<NSAttributedString, Shared>>;
        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(&self, attributedTitle: Option<&NSAttributedString>);
        #[method(isSeparatorItem)]
        pub unsafe fn isSeparatorItem(&self) -> bool;
        #[method_id(keyEquivalent)]
        pub unsafe fn keyEquivalent(&self) -> Id<NSString, Shared>;
        #[method(setKeyEquivalent:)]
        pub unsafe fn setKeyEquivalent(&self, keyEquivalent: &NSString);
        #[method(keyEquivalentModifierMask)]
        pub unsafe fn keyEquivalentModifierMask(&self) -> NSEventModifierFlags;
        #[method(setKeyEquivalentModifierMask:)]
        pub unsafe fn setKeyEquivalentModifierMask(
            &self,
            keyEquivalentModifierMask: NSEventModifierFlags,
        );
        #[method_id(userKeyEquivalent)]
        pub unsafe fn userKeyEquivalent(&self) -> Id<NSString, Shared>;
        #[method(allowsKeyEquivalentWhenHidden)]
        pub unsafe fn allowsKeyEquivalentWhenHidden(&self) -> bool;
        #[method(setAllowsKeyEquivalentWhenHidden:)]
        pub unsafe fn setAllowsKeyEquivalentWhenHidden(&self, allowsKeyEquivalentWhenHidden: bool);
        #[method(allowsAutomaticKeyEquivalentLocalization)]
        pub unsafe fn allowsAutomaticKeyEquivalentLocalization(&self) -> bool;
        #[method(setAllowsAutomaticKeyEquivalentLocalization:)]
        pub unsafe fn setAllowsAutomaticKeyEquivalentLocalization(
            &self,
            allowsAutomaticKeyEquivalentLocalization: bool,
        );
        #[method(allowsAutomaticKeyEquivalentMirroring)]
        pub unsafe fn allowsAutomaticKeyEquivalentMirroring(&self) -> bool;
        #[method(setAllowsAutomaticKeyEquivalentMirroring:)]
        pub unsafe fn setAllowsAutomaticKeyEquivalentMirroring(
            &self,
            allowsAutomaticKeyEquivalentMirroring: bool,
        );
        #[method_id(image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);
        #[method(state)]
        pub unsafe fn state(&self) -> NSControlStateValue;
        #[method(setState:)]
        pub unsafe fn setState(&self, state: NSControlStateValue);
        #[method_id(onStateImage)]
        pub unsafe fn onStateImage(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setOnStateImage:)]
        pub unsafe fn setOnStateImage(&self, onStateImage: Option<&NSImage>);
        #[method_id(offStateImage)]
        pub unsafe fn offStateImage(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setOffStateImage:)]
        pub unsafe fn setOffStateImage(&self, offStateImage: Option<&NSImage>);
        #[method_id(mixedStateImage)]
        pub unsafe fn mixedStateImage(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setMixedStateImage:)]
        pub unsafe fn setMixedStateImage(&self, mixedStateImage: Option<&NSImage>);
        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;
        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);
        #[method(isAlternate)]
        pub unsafe fn isAlternate(&self) -> bool;
        #[method(setAlternate:)]
        pub unsafe fn setAlternate(&self, alternate: bool);
        #[method(indentationLevel)]
        pub unsafe fn indentationLevel(&self) -> NSInteger;
        #[method(setIndentationLevel:)]
        pub unsafe fn setIndentationLevel(&self, indentationLevel: NSInteger);
        #[method_id(target)]
        pub unsafe fn target(&self) -> Option<Id<Object, Shared>>;
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);
        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;
        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);
        #[method(tag)]
        pub unsafe fn tag(&self) -> NSInteger;
        #[method(setTag:)]
        pub unsafe fn setTag(&self, tag: NSInteger);
        #[method_id(representedObject)]
        pub unsafe fn representedObject(&self) -> Option<Id<Object, Shared>>;
        #[method(setRepresentedObject:)]
        pub unsafe fn setRepresentedObject(&self, representedObject: Option<&Object>);
        #[method_id(view)]
        pub unsafe fn view(&self) -> Option<Id<NSView, Shared>>;
        #[method(setView:)]
        pub unsafe fn setView(&self, view: Option<&NSView>);
        #[method(isHighlighted)]
        pub unsafe fn isHighlighted(&self) -> bool;
        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;
        #[method(setHidden:)]
        pub unsafe fn setHidden(&self, hidden: bool);
        #[method(isHiddenOrHasHiddenAncestor)]
        pub unsafe fn isHiddenOrHasHiddenAncestor(&self) -> bool;
        #[method_id(toolTip)]
        pub unsafe fn toolTip(&self) -> Option<Id<NSString, Shared>>;
        #[method(setToolTip:)]
        pub unsafe fn setToolTip(&self, toolTip: Option<&NSString>);
    }
);
extern_methods!(
    #[doc = "NSViewEnclosingMenuItem"]
    unsafe impl NSView {
        #[method_id(enclosingMenuItem)]
        pub unsafe fn enclosingMenuItem(&self) -> Option<Id<NSMenuItem, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSMenuItem {
        #[method(setMnemonicLocation:)]
        pub unsafe fn setMnemonicLocation(&self, location: NSUInteger);
        #[method(mnemonicLocation)]
        pub unsafe fn mnemonicLocation(&self) -> NSUInteger;
        #[method_id(mnemonic)]
        pub unsafe fn mnemonic(&self) -> Option<Id<NSString, Shared>>;
        #[method(setTitleWithMnemonic:)]
        pub unsafe fn setTitleWithMnemonic(&self, stringWithAmpersand: &NSString);
    }
);
