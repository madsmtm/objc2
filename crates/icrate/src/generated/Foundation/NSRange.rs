#[doc = "NSValueRangeExtensions"]
impl NSValue {
    pub unsafe fn valueWithRange(range: NSRange) -> Id<NSValue, Shared> {
        msg_send_id![Self::class(), valueWithRange: range]
    }
    pub unsafe fn rangeValue(&self) -> NSRange {
        msg_send![self, rangeValue]
    }
}
