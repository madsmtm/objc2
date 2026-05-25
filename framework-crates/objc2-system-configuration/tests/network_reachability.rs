#![cfg(all(feature = "SCNetworkReachability", feature = "libc"))]
#![allow(deprecated)]
use objc2_system_configuration::{SCNetworkReachability, SCNetworkReachabilityFlags};
use std::{
    ffi::CString,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
};

#[must_use]
fn is_reachable(reachability: &SCNetworkReachability) -> bool {
    let mut flags = SCNetworkReachabilityFlags::empty();
    assert!(reachability.flags(&mut flags));
    flags.contains(SCNetworkReachabilityFlags::Reachable)
}

#[test]
fn local_network_reachability() {
    let sockaddrs: &[IpAddr] = &[Ipv6Addr::LOCALHOST.into(), Ipv4Addr::LOCALHOST.into()];

    for ip in sockaddrs {
        let addr = SocketAddr::new(*ip, 0);
        let reachability = SCNetworkReachability::with_address(None, addr).unwrap();
        assert!(is_reachable(&reachability));
    }
}

#[test]
fn any_to_local_sockaddr_pair_reachability() {
    let pairs: &[(IpAddr, IpAddr)] = &[
        (Ipv4Addr::UNSPECIFIED.into(), Ipv4Addr::LOCALHOST.into()),
        (Ipv4Addr::UNSPECIFIED.into(), Ipv6Addr::LOCALHOST.into()),
        (Ipv6Addr::UNSPECIFIED.into(), Ipv4Addr::LOCALHOST.into()),
        (Ipv6Addr::UNSPECIFIED.into(), Ipv6Addr::LOCALHOST.into()),
    ];

    for (local, remote) in pairs {
        let local = SocketAddr::new(*local, 0);
        let remote = SocketAddr::new(*remote, 0);
        let reachability =
            SCNetworkReachability::with_address_pair(None, Some(local), Some(remote)).unwrap();
        assert!(is_reachable(&reachability));
    }
}

#[test]
fn optional_sockaddr_pair_reachability() {
    assert_eq!(
        SCNetworkReachability::with_address_pair(None, None, None),
        None
    );

    let reachability = SCNetworkReachability::with_address_pair(
        None,
        None,
        Some(SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 0)),
    )
    .unwrap();
    assert!(is_reachable(&reachability));

    // Reachability here depends on network status.
    let _reachability = SCNetworkReachability::with_address_pair(
        None,
        Some(SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 0)),
        None,
    )
    .unwrap();
}

#[test]
fn dns_google() {
    let sockaddrs: &[SocketAddr] = &[
        "[2001:4860:4860::8844]:443".parse().unwrap(),
        "8.8.4.4:443".parse().unwrap(),
    ];
    for remote in sockaddrs {
        // Verify that status of TCP connection is reported the same as
        // reachability.
        //
        // This is inherently race-prone, but it probably won't be a big
        // issue, Google's DNS is likely pretty stable.
        match std::net::TcpStream::connect(*remote) {
            Ok(tcp) => {
                let local = tcp.local_addr().unwrap();
                let remote = tcp.peer_addr().unwrap();
                let reachability =
                    SCNetworkReachability::with_address_pair(None, Some(local), Some(remote))
                        .unwrap();
                assert!(is_reachable(&reachability));
            }
            Err(_) => {
                let reachability =
                    SCNetworkReachability::with_address_pair(None, None, Some(*remote)).unwrap();
                assert!(!is_reachable(&reachability));
            }
        }
    }
}

#[test]
fn name() {
    assert!(SCNetworkReachability::with_name(None, &CString::new("").unwrap()).is_none());

    // Reachability status depends on network connection.
    let _ = SCNetworkReachability::with_name(None, &CString::new("example.com").unwrap()).unwrap();
    let _ = SCNetworkReachability::with_name(None, &CString::new("something").unwrap()).unwrap();
    let _ = SCNetworkReachability::with_name(None, &CString::new("en0").unwrap()).unwrap();
}
