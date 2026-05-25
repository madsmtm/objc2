#[cfg(feature = "libc")]
use crate::SCNetworkConnectionFlags;

/// Determines if the given network address is reachable using the current
/// network configuration.
///
/// Note: this API has been deprecated but you can get equivalent results with:
/// ```ignore
/// let reachability = SCNetworkReachability::with_address(None, address);
/// let mut flags = SCNetworkReachabilityFlags::empty();
/// assert!(reachability.flags(&mut flags));
/// ```
///
/// Parameter `address`: The network address of the desired host.
///
/// Parameter `flags`: A pointer to memory that will be filled with a
/// set of SCNetworkConnectionFlags detailing the reachability
/// of the specified address.
///
/// Returns: Returns TRUE if the network connection flags are valid;
/// FALSE if the status could not be determined.
#[cfg(feature = "libc")]
#[deprecated = "No longer supported"]
#[allow(deprecated)]
#[inline]
#[allow(non_snake_case)]
pub fn SCNetworkCheckReachabilityByAddress(
    address: crate::sockaddr::SocketAddr,
    flags: &mut SCNetworkConnectionFlags,
) -> bool {
    crate::sockaddr::with_c_sockaddr(address, |c_addr| {
        let len = unsafe { (*c_addr.as_ptr()).sa_len };
        // SAFETY: The address pointer is valid.
        unsafe { crate::__SCNetworkCheckReachabilityByAddress(c_addr, len as _, flags) }
    })
}
