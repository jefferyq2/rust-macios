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

use super::traits::{INSButton, INSControl, INSResponder, INSView};

/// A control that defines an area on the screen that a user clicks to trigger an action.
#[repr(transparent)]
pub struct NSButton {
    /// The underlying `NSButton`.
    pub ptr: Id<Object>,
}

impl PNSObject for NSButton {
    fn im_class<'a>() -> &'a Class {
        class!(NSButton)
    }

    fn im_self(&self) -> id {
        unsafe { msg_send![&*self.ptr, self] }
    }
}

impl INSView for NSButton {}

impl INSControl for NSButton {}

impl INSButton for NSButton {}

impl fmt::Debug for NSButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}

impl INSResponder for NSButton {}

impl ToId for NSButton {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSButton {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}
