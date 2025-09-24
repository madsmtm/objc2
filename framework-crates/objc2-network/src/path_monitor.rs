use core::ptr::NonNull;

use crate::{generated::*, nw_path_monitor_create, InterfaceType, NWRetained};

nw_object!(
    #[doc(alias = "nw_path_monitor_t")]
    pub struct PathMonitor;
);

impl PathMonitor {
    pub fn new() -> NWRetained<Self> {
        nw_path_monitor_create()
    }

    pub fn new_with_interface_type(interface_type: InterfaceType) -> NWRetained<Self> {
        nw_path_monitor_create_with_type(interface_type)
    }

    pub fn prohibit_interface_type(&mut self, interface_type: InterfaceType) {
        unsafe {
            nw_path_monitor_prohibit_interface_type(
                // fixme: should be able to use &PathMonitor?
                NonNull::from(self),
                interface_type,
            );
        }
    }

    pub fn set_queue(&mut self, queue: &dispatch2::DispatchQueue) {
        unsafe {
            nw_path_monitor_set_queue(
                // fixme: should be able to use &PathMonitor?
                NonNull::from(self),
                queue,
            );
        }
    }

    pub fn set_update_handler(&mut self, update_handler: nw_path_monitor_update_handler_t) {
        unsafe {
            nw_path_monitor_set_update_handler(
                // fixme: should be able to use &PathMonitor?
                NonNull::from(self),
                update_handler,
            );
        }
    }

    pub fn set_cancel_handler(&mut self, cancel_handler: nw_path_monitor_cancel_handler_t) {
        unsafe {
            nw_path_monitor_set_cancel_handler(
                // fixme: should be able to use &PathMonitor?
                NonNull::from(self),
                cancel_handler,
            );
        }
    }

    pub fn start(&mut self) {
        unsafe {
            nw_path_monitor_start(
                // fixme: should be able to use &PathMonitor?
                NonNull::from(self),
            );
        }
    }

    pub fn cancel(&mut self) {
        unsafe {
            nw_path_monitor_cancel(
                // fixme: should be able to use &PathMonitor?
                NonNull::from(self),
            );
        }
    }
}
