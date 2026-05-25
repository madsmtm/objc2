use crate::NWEndpoint;

impl NWEndpoint {
    /// Creates a network endpoint from a socket address.
    ///
    /// The endpoint will have the type `nw_endpoint_type_address`.
    #[doc(alias = "nw_endpoint_create_address")]
    #[cfg(feature = "libc")]
    #[inline]
    pub fn new_address(address: crate::sockaddr::SocketAddr) -> crate::NWRetained<NWEndpoint> {
        // SAFETY: The address is a valid `sockaddr` pointer.
        crate::sockaddr::with_c_sockaddr(address, |c_addr| unsafe { Self::__new_address(c_addr) })
    }

    /// Retrieves the socket address for a network endpoint with the type
    /// `nw_endpoint_type_address`.
    #[doc(alias = "nw_endpoint_get_address")]
    #[cfg(feature = "libc")]
    #[inline]
    pub fn address(&self) -> crate::sockaddr::SocketAddr {
        // SAFETY: The address is a valid `sockaddr` pointer, and should be
        // alive for long enough for us to copy it into `SocketAddr` here.
        unsafe { crate::sockaddr::from_c_sockaddr(self.__address()) }
    }
}

#[cfg(test)]
#[cfg(feature = "libc")]
mod tests {
    use core::str::FromStr;
    use std::ffi::CString;

    use super::*;
    use crate::sockaddr::*;

    #[test]
    fn roundtrip() {
        let addrs: &[SocketAddr] = &[
            SocketAddrV4::new(Ipv4Addr::LOCALHOST, 42).into(),
            SocketAddrV6::new(Ipv6Addr::LOCALHOST, 123, 456, 789).into(),
        ];
        for addr in addrs {
            let roundtripped = NWEndpoint::new_address(*addr).address();
            assert_eq!(roundtripped, *addr);
        }
    }

    #[test]
    fn address_from_not_same_type() {
        let endpoint = NWEndpoint::new_host(
            &CString::new("127.0.0.1").unwrap(),
            &CString::new("42").unwrap(),
        );
        assert_eq!(
            endpoint.address(),
            SocketAddr::from_str("127.0.0.1:42").unwrap()
        );
    }
}
