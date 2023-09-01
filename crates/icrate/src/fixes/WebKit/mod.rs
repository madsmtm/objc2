use crate::common::*;
#[cfg(feature = "WebKit_WKNavigationAction")]
use crate::WebKit::*;

extern_methods!(
    #[cfg(feature = "WebKit_WKNavigationAction")]
    unsafe impl WKNavigationAction {
        #[cfg(feature = "WebKit_WKFrameInfo")]
        #[method_id(@__retain_semantics Other sourceFrame)]
        pub unsafe fn sourceFrame(&self) -> Option<Id<WKFrameInfo>>;
    }
);
