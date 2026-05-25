// This file is symlinked to `objc2-system-configuration` too.
#![allow(dead_code)]

// TODO(MSRV): core::net introduced in 1.77
#[cfg(not(feature = "std"))]
pub(crate) use core::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
#[cfg(feature = "std")]
pub(crate) use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use core::ffi::c_int;
use core::mem::size_of;
use core::ptr::{self, NonNull};

/// Convert the given [`SocketAddr`] to a [`libc::sockaddr_in`] pointer or a
/// [`libc::sockaddr_in6`] pointer, and pass that to the given closure.
pub(crate) fn with_c_sockaddr<R>(
    addr: SocketAddr,
    closure: impl FnOnce(NonNull<libc::sockaddr>) -> R,
) -> R {
    #[cfg(target_vendor = "apple")] // sockaddr contains size internally here
    match addr {
        // See reference conversion from socket2:
        // https://github.com/rust-lang/socket2/blob/v0.6.3/src/sockaddr.rs#L352-L363
        // https://github.com/rust-lang/socket2/blob/v0.6.3/src/sys/unix.rs#L1365-L1372
        SocketAddr::V4(addr) => closure(
            NonNull::from(&libc::sockaddr_in {
                sin_len: size_of::<libc::sockaddr_in>() as u8,
                sin_family: libc::AF_INET as libc::sa_family_t,
                sin_port: addr.port().to_be(),
                sin_addr: {
                    // `s_addr` is stored as BE on all machines, and the array
                    // is in BE order. So the native endian conversion method
                    // is used so that it's never swapped.
                    libc::in_addr {
                        s_addr: u32::from_ne_bytes(addr.ip().octets()),
                    }
                },
                sin_zero: Default::default(),
            })
            .cast(),
        ),
        // See reference conversion from socket2:
        // https://github.com/rust-lang/socket2/blob/v0.6.3/src/sockaddr.rs#L388-L407
        // https://github.com/rust-lang/socket2/blob/v0.6.3/src/sys/unix.rs#L1378-L1382
        SocketAddr::V6(addr) => closure(
            NonNull::from(&libc::sockaddr_in6 {
                sin6_len: size_of::<libc::sockaddr_in6>() as u8,
                sin6_family: libc::AF_INET6 as libc::sa_family_t,
                sin6_port: addr.port().to_be(),
                sin6_flowinfo: addr.flowinfo(),
                sin6_addr: libc::in6_addr {
                    s6_addr: addr.ip().octets(),
                },
                sin6_scope_id: addr.scope_id(),
            })
            .cast(),
        ),
    }
    #[cfg(not(target_vendor = "apple"))]
    {
        let _ = addr;
        let _ = closure;
        panic!("unsupported target");
    }
}

pub(crate) fn with_nullable_c_sockaddr<R>(
    addr: Option<SocketAddr>,
    closure: impl FnOnce(*const libc::sockaddr) -> R,
) -> R {
    if let Some(addr) = addr {
        with_c_sockaddr(addr, |addr| closure(addr.as_ptr()))
    } else {
        closure(ptr::null())
    }
}

/// Convert a pointer to a [`libc::sockaddr`] to a [`SocketAddr`].
///
/// # Safety
///
/// The pointer must point to a valid `sockaddr_in` or `sockaddr_in6`.
pub(crate) unsafe fn from_c_sockaddr(addr: NonNull<libc::sockaddr>) -> SocketAddr {
    // Based upon `std::net` conversions:
    // https://github.com/rust-lang/rust/blob/1.95.0/library/std/src/sys/net/connection/socket/mod.rs#L93-L208
    //
    // A bit different because Apple's `sockaddr` types store the length.

    // SAFETY: Caller upholds that the pointer is valid.
    let len = unsafe { (*addr.as_ptr()).sa_len };

    // SAFETY: Caller upholds that the pointer is valid.
    match unsafe { (*addr.as_ptr()).sa_family } as c_int {
        libc::AF_INET => {
            assert!(len >= size_of::<libc::sockaddr_in>() as u8);
            // SAFETY: We just checked that the family is IPv4.
            let addr = unsafe { addr.cast::<libc::sockaddr_in>().as_ref() };
            SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr::from(addr.sin_addr.s_addr.to_ne_bytes()),
                u16::from_be(addr.sin_port),
            ))
        }
        libc::AF_INET6 => {
            assert!(len >= size_of::<libc::sockaddr_in6>() as u8);
            // SAFETY: We just checked that the family is IPv6.
            let addr = unsafe { addr.cast::<libc::sockaddr_in6>().as_ref() };
            SocketAddr::V6(SocketAddrV6::new(
                Ipv6Addr::from(addr.sin6_addr.s6_addr),
                u16::from_be(addr.sin6_port),
                addr.sin6_flowinfo,
                addr.sin6_scope_id,
            ))
        }
        family => panic!("invalid socket address family: {family:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let addrs: &[SocketAddr] = &[
            SocketAddrV4::new(Ipv4Addr::LOCALHOST, 42).into(),
            SocketAddrV6::new(Ipv6Addr::LOCALHOST, 123, 456, 789).into(),
        ];
        for addr in addrs {
            let roundtripped = with_c_sockaddr(*addr, |c_addr| unsafe { from_c_sockaddr(c_addr) });
            assert_eq!(roundtripped, *addr);
        }
    }
}
