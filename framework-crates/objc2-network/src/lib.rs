//! # Bindings to the `Network` framework
//!
//! See [Apple's docs][apple-doc] and [the general docs on framework crates][framework-crates] for more information.
//!
//! [apple-doc]: https://developer.apple.com/documentation/network/
//! [framework-crates]: https://docs.rs/objc2/latest/objc2/topics/about_generated/index.html
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2-network/0.3.1")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod generated;
#[macro_use]
mod macros;
mod advertise_descriptor;
mod browse_result;
mod browser;
mod browser_descriptor;
mod connection;
mod connection_group;
mod connection_report;
mod content_context;
mod endpoint;
mod error;
mod ethernet_channel;
mod framer_options;
mod group_descriptor;
mod interface;
mod listener;
mod object;
mod parameters;
mod path;
mod path_monitor;
mod privacy_context;
mod protocol_options;
mod proxy_config;
mod resolver_config;
mod retained;
mod txt_record;
mod ws_options;

use core::{
    cell::UnsafeCell,
    marker::{PhantomData, PhantomPinned},
};

#[allow(unused_imports, unreachable_pub)]
pub use self::generated::*;
pub use self::retained::NWRetained;
pub use object::NWObject;

pub use advertise_descriptor::AdvertiseDescriptor;
pub use browse_result::BrowseResult;
pub use browser::Browser;
pub use browser_descriptor::BrowseDescriptor;
pub use connection::Connection;
pub use connection_group::ConnectionGroup;
pub use connection_report::{DataTransferReport, EstablishmentReport, ResolutionReport};
pub use content_context::ContentContext;
pub use endpoint::Endpoint;
pub use error::Error;
pub use ethernet_channel::EthernetChannel;
pub use framer_options::Framer;
pub use group_descriptor::GroupDescriptor;
pub use interface::Interface;
pub use listener::Listener;
pub use parameters::{Parameters, ProtocolStack};
pub use path::Path;
pub use path_monitor::PathMonitor;
pub use privacy_context::PrivacyContext;
pub use protocol_options::{ProtocolDefinition, ProtocolMetadata, ProtocolOptions};
pub use proxy_config::{ProxyConfig, RelayHop};
pub use resolver_config::ResolverConfig;
pub use txt_record::TxtRecord;
pub use ws_options::{WsRequest, WsResponse};

// Generated types that should be made public
pub use generated::{nw_path_monitor_cancel_handler_t, nw_path_monitor_update_handler_t};

// Friendly type-aliases for generated types
pub use generated::{nw_interface_type_t as InterfaceType, nw_path_status_t as PathStatus};

// Helper type
type OpaqueData = UnsafeCell<PhantomData<(*const UnsafeCell<()>, PhantomPinned)>>;

// Type aliases for generated types
#[allow(non_camel_case_types)]
type nw_advertise_descriptor_t = AdvertiseDescriptor;
#[allow(non_camel_case_types)]
type nw_browse_result_t = BrowseResult;
#[allow(non_camel_case_types)]
type nw_browse_descriptor_t = BrowseDescriptor;
#[allow(non_camel_case_types)]
type nw_connection_t = Connection;
#[allow(non_camel_case_types)]
type nw_data_transfer_report_t = DataTransferReport;
#[allow(non_camel_case_types)]
type nw_establishment_report_t = EstablishmentReport;
#[allow(non_camel_case_types)]
type nw_resolution_report_t = ResolutionReport;
#[allow(non_camel_case_types)]
type nw_endpoint_t = Endpoint;
#[allow(non_camel_case_types)]
type nw_ethernet_channel_t = EthernetChannel;
#[allow(non_camel_case_types)]
type nw_framer_t = Framer;
#[allow(non_camel_case_types)]
type nw_group_descriptor_t = GroupDescriptor;
#[allow(non_camel_case_types)]
type nw_interface_t = Interface;
#[allow(non_camel_case_types)]
type nw_parameters_t = Parameters;
#[allow(non_camel_case_types)]
type nw_protocol_stack_t = ProtocolStack;
#[allow(non_camel_case_types)]
type nw_path_t = Path;
#[allow(non_camel_case_types)]
type nw_path_monitor_t = PathMonitor;
#[allow(non_camel_case_types)]
type nw_privacy_context_t = PrivacyContext;
#[allow(non_camel_case_types)]
type nw_protocol_definition_t = ProtocolDefinition;
#[allow(non_camel_case_types)]
type nw_protocol_metadata_t = ProtocolMetadata;
#[allow(non_camel_case_types)]
type nw_protocol_options_t = ProtocolOptions;
#[allow(non_camel_case_types)]
type nw_proxy_config_t = ProxyConfig;
#[allow(non_camel_case_types)]
type nw_relay_hop_t = RelayHop;
#[allow(non_camel_case_types)]
type nw_resolver_config_t = ResolverConfig;
#[allow(non_camel_case_types)]
type nw_txt_record_t = TxtRecord;
#[allow(non_camel_case_types)]
type nw_error_t = Error;
#[allow(non_camel_case_types)]
type nw_ws_response_t = WsResponse;
#[allow(non_camel_case_types)]
type nw_ws_request_t = WsRequest;
#[allow(non_camel_case_types)]
type nw_listener_t = Listener;
#[allow(non_camel_case_types)]
type nw_browser_t = Browser;
#[allow(non_camel_case_types)]
type nw_connection_group_t = ConnectionGroup;
#[allow(non_camel_case_types)]
type nw_content_context_t = ContentContext;

// fixme: Generated as ArrayUnknownABI
#[allow(non_camel_case_types)]
pub type nw_ethernet_address_t = [core::ffi::c_uchar; 6];
