#![no_std]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

//! Diagnostics over Internet Protocol definition library
//!
//! This crate is built to act a definitive source for defintions of ISO-13400
//! and `DoIP` protocol development. Using all the data structures provided one
//! should be able to develop applications with certainty that they are abiding
//! by regulatory standards.
//!
//! Each `DoIP` message contains a minimum length of 8 bytes of which is the
//! [`header`], within this the header contains the length of the [`message`]
//! and what type of payload the message is.
//!
//! Due to `DoIP` being a networking protocol types such as `LogicalAddress` have been
//! kept to `[u8; 2]` rather than `u16`, this is to remain as close as possible
//! to real situations of on-wire communication.

/// Contains header related logic and data structures.
///
/// The `DoIP` header contains 4 items:
/// - Protocol Version
/// - Inverse Protocol Version
/// - Payload Type
/// - Payload Length
///
/// By default this library provides users with the option to use a different
/// protocol version than what is currently in use, this is for backwards and future
/// compatability.
///
/// The Inverse Protocol Version is the flipped byte of the protocol
/// version, this is to validate the packet.
///
/// The payload length is given in a `u32` but in this library is written in
/// `[u8; 4]` to remain close-to-network as possible.
///
/// This module contains the relevant structs, enums, and traits for developers
/// to create custom headers.
pub mod header {
    pub use super::doip_header::payload_type::*;
    pub use super::doip_header::version::*;
    pub use super::doip_header::DoipHeader;
}

/// Contains message data structures and internal payload type dependant structures.
pub mod payload {
    pub use super::doip_payload::action_code::*;
    pub use super::doip_payload::activation_code::*;
    pub use super::doip_payload::activation_type::*;
    pub use super::doip_payload::diagnostic_ack::*;
    pub use super::doip_payload::diagnostic_nack::*;
    pub use super::doip_payload::message::*;
    pub use super::doip_payload::nack_code::*;
    pub use super::doip_payload::node_type::*;
    pub use super::doip_payload::power_mode::*;
    pub use super::doip_payload::sync_status::*;

    pub use super::doip_payload::alive_check_request::*;
    pub use super::doip_payload::alive_check_response::*;
    pub use super::doip_payload::diagnostic_message::*;
    pub use super::doip_payload::diagnostic_message_ack::*;
    pub use super::doip_payload::diagnostic_message_nack::*;
    pub use super::doip_payload::entity_status_request::*;
    pub use super::doip_payload::entity_status_response::*;
    pub use super::doip_payload::generic_nack::*;
    pub use super::doip_payload::power_information_request::*;
    pub use super::doip_payload::power_information_response::*;
    pub use super::doip_payload::routing_activation_request::*;
    pub use super::doip_payload::routing_activation_response::*;
    pub use super::doip_payload::vehicle_announcement_message::*;
    pub use super::doip_payload::vehicle_identification_request::*;
    pub use super::doip_payload::vehicle_identification_request_eid::*;
    pub use super::doip_payload::vehicle_identification_request_vin::*;
}

/// Implemented across `DoIP` Payload Types for consistent encoding and decoding of buffers.
///
/// `DoipPayload` is implemented across all the `DoIP` Payload Types for the
/// purpose of consistent encoding and decoding as well as identification within
/// a buffer.
pub trait DoipPayload: core::fmt::Debug + Send {
    /// Used to identify the payload self for `DoipHeader` construction.
    fn payload_type(&self) -> header::PayloadType;
}

/// Contains all constants used in ISO-13400.
///
/// The definitions found here are originally from Wireshark's repository. Wireshark
/// is a packet sniffing tool with an already supported `DoIP` and UDS packet disassembly
/// and so their definitions were lifted so to support this crate.
pub mod definitions;

mod doip_header;
mod doip_payload;
