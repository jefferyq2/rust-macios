use std::fmt;

use objc::{
    class, msg_send,
    runtime::{Class, Object},
    sel, sel_impl,
};
use objc_id::Id;

use crate::objective_c_runtime::{
    id,
    traits::{FromId, PNSObject, ToId},
};

use super::traits::{INSButton, INSControl, INSResponder, INSStatusBarButton, INSView};

/// The appearance and behavior of an item in the systemwide menu bar.
pub struct NSStatusBarButton {
    /// The undrlying Objective-C object.
    pub ptr: Id<Object>,
}
impl PNSObject for NSStatusBarButton {
    fn im_class<'a>() -> &'a Class {
        class!(NSStatusBarButton)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![self.ptr, self] }
    }
}

impl INSResponder for NSStatusBarButton {}

impl INSView for NSStatusBarButton {}

impl INSControl for NSStatusBarButton {}

impl INSButton for NSStatusBarButton {}

impl INSStatusBarButton for NSStatusBarButton {}

impl fmt::Debug for NSStatusBarButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSStatusBarButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl ToId for NSStatusBarButton {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSStatusBarButton {
    unsafe fn from_id(ptr: id) -> Self {
        NSStatusBarButton {
            ptr: Id::from_ptr(ptr),
        }
    }
}
