use core::ptr::NonNull;

use crate::{generated::*, InterfaceType, PathStatus};

nw_object!(
    #[doc(alias = "nw_path_t")]
    pub struct Path;
);

impl Path {
    // todo: turn nw_path_status_t into enum?
    // fixme: does not need to be mut but NonNull wants mut
    pub fn status(&mut self) -> PathStatus {
        unsafe {
            // fixme: should be able to use &Path?
            nw_path_get_status(NonNull::from(self))
        }
    }

    pub fn is_equal_to_path(&mut self, other: &mut Self) -> bool {
        unsafe {
            // fixme: should be able to use &Path?
            nw_path_is_equal(NonNull::from(self), NonNull::from(other))
        }
    }

    pub fn uses_interface_type(&mut self, interface_type: InterfaceType) -> bool {
        unsafe {
            // fixme: should be able to use &Path?
            nw_path_uses_interface_type(NonNull::from(self), interface_type)
        }
    }

    pub fn supports_ipv4(&mut self) -> bool {
        unsafe {
            // fixme: should be able to use &Path?
            nw_path_has_ipv4(NonNull::from(self))
        }
    }

    pub fn supports_ipv6(&mut self) -> bool {
        unsafe {
            // fixme: should be able to use &Path?
            nw_path_has_ipv6(NonNull::from(self))
        }
    }

    pub fn supports_dns(&mut self) -> bool {
        unsafe {
            // fixme: should be able to use &Path?
            nw_path_has_dns(NonNull::from(self))
        }
    }

    pub fn is_expensive(&mut self) -> bool {
        unsafe {
            // fixme: should be able to use &Path?
            nw_path_is_expensive(NonNull::from(self))
        }
    }

    pub fn is_constrained(&mut self) -> bool {
        unsafe {
            // fixme: should be able to use &Path?
            nw_path_is_constrained(NonNull::from(self))
        }
    }
}
