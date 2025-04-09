dispatch_object!(
    /// Dispatch data.
    #[doc(alias = "dispatch_data_t")]
    #[doc(alias = "dispatch_data_s")]
    pub struct DispatchData;
);

#[cfg(test)]
mod tests {
    use super::*;

    // Intentionally not Send + Sync, as `DispatchData` can contain things
    // like `NSData`.
    static_assertions::assert_not_impl_any!(DispatchData: Send, Sync);
}
