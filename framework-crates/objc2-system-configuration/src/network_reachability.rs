use crate::SCNetworkReachability;

impl SCNetworkReachability {
    /// Creates a reference to the specified network address.
    ///
    /// This reference can be used later to monitor the reachability of the
    /// target host.
    ///
    /// Parameter `local`: The local address associated with a network
    /// connection.  If `None`, only the remote address is of interest.
    ///
    /// Parameter `remote`: The remote address associated with a network
    /// connection.  If `None`, only the local address is of interest.
    #[doc(alias = "SCNetworkReachabilityCreateWithAddressPair")]
    #[cfg(feature = "libc")]
    #[deprecated]
    #[allow(deprecated)]
    pub fn with_address_pair(
        allocator: Option<&objc2_core_foundation::CFAllocator>,
        local: Option<crate::sockaddr::SocketAddr>,
        remote: Option<crate::sockaddr::SocketAddr>,
    ) -> Option<objc2_core_foundation::CFRetained<Self>> {
        crate::sockaddr::with_nullable_c_sockaddr(local, |local| {
            crate::sockaddr::with_nullable_c_sockaddr(remote, |remote| {
                // SAFETY: The local and remote addresses are valid `sockaddr`
                // pointers or NULL.
                unsafe { Self::__with_address_pair(allocator, local, remote) }
            })
        })
    }

    /// Creates a reference to the specified network address.
    ///
    /// This reference can be used later to monitor the reachability of the
    /// target host.
    #[doc(alias = "SCNetworkReachabilityCreateWithAddress")]
    #[cfg(feature = "libc")]
    #[deprecated]
    #[allow(deprecated)]
    pub fn with_address(
        allocator: Option<&objc2_core_foundation::CFAllocator>,
        address: crate::sockaddr::SocketAddr,
    ) -> Option<objc2_core_foundation::CFRetained<Self>> {
        // SAFETY: The address is a valid `sockaddr` pointer.
        crate::sockaddr::with_c_sockaddr(address, |address| unsafe {
            Self::__with_address(allocator, address)
        })
    }
}
