#![allow(non_snake_case)]
pub type MXLaunchTaskID = objc2_foundation::NSString;

// pub type os_log_t = ProtocolObject<dyn OS_os_log>;

// extern_protocol!(
//     pub unsafe trait OS_os_log: NSObjectProtocol {}
// );

impl crate::MXMetricManager {
    objc2::extern_methods!(
        #[unsafe(method(extendLaunchMeasurementForTaskID:error:_))]
        pub unsafe fn extendLaunchMeasurementForTaskID_error(
            &self,
            task_id: &MXLaunchTaskID,
        ) -> Result<(), objc2::rc::Retained<objc2_foundation::NSError>>;

        #[unsafe(method(finishExtendedLaunchMeasurementForTaskID:error:_))]
        pub unsafe fn finishExtendedLaunchMeasurementForTaskID_error(
            &self,
            task_id: &MXLaunchTaskID,
        ) -> Result<(), objc2::rc::Retained<objc2_foundation::NSError>>;
    );

    // #[unsafe(method(makeLogHandleWithCategory:))]
    // pub unsafe fn makeLogHandleWithCategory(category: &objc2_foundation::NSString) -> Retained<os_log_t>;
}
