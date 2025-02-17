#![no_std]
#![warn(clippy::pedantic)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![allow(clippy::module_name_repetitions)]

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

use definitions::{
    DOIP_COMMON_EID_LEN, DOIP_COMMON_VIN_LEN, DOIP_DIAG_COMMON_SOURCE_LEN,
    DOIP_DIAG_COMMON_TARGET_LEN, DOIP_ENTITY_STATUS_RESPONSE_LEN, DOIP_HEADER_LEN,
    DOIP_ROUTING_ACTIVATION_REQ_LEN, DOIP_ROUTING_ACTIVATION_RES_LEN,
    DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG,
};
use header::DoipHeader;
use payload::DoipPayload;

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
    pub use super::doip_payload::nack_code::*;
    pub use super::doip_payload::node_type::*;
    pub use super::doip_payload::power_mode::*;
    pub use super::doip_payload::sync_status::*;
    pub use super::doip_payload::DoipPayload;

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

/// Contains all constants used in ISO-13400.
///
/// The definitions found here are originally from Wireshark's repository. Wireshark
/// is a packet sniffing tool with an already supported `DoIP` and UDS packet disassembly
/// and so their definitions were lifted so to support this crate.
pub mod definitions;

mod doip_header;
mod doip_payload;

/// The decoded struct of a `DoIP` packet.
///
/// Each `DoIP` packet contains a header which describes the message, this is outlined
/// in `DoipHeader`.
///
/// Some Payload Types available in `DoIP` require a payload which is covered by
/// `DoipPayload`.
#[derive(Debug)]
pub struct DoipMessage<'a> {
    /// Defined by `DoipHeader`, the header supplies the information for programs
    /// to understand the payload.
    pub header: DoipHeader,

    /// Takes any struct implementing `DoipPayload`.
    pub payload: DoipPayload<'a>,
}

impl<'a, const N: usize> From<DoipMessage<'a>> for [u8; N] {
    fn from(value: DoipMessage<'a>) -> Self {
        let header = <[u8; DOIP_HEADER_LEN]>::from(value.header);

        let mut buffer = [0u8; N];
        let mut offset = 0;

        buffer[offset..offset + DOIP_HEADER_LEN].copy_from_slice(&header);
        offset += DOIP_HEADER_LEN;

        // Match on the payload and convert each variant to the appropriate byte slice
        match value.payload {
            DoipPayload::GenericNack(generic_nack) => {
                let bytes = <[u8; 1]>::from(generic_nack);
                buffer[offset..].copy_from_slice(&bytes);

                buffer
            }
            DoipPayload::VehicleIdentificationRequest(vehicle_identification_request) => {
                let bytes = <[u8; 0]>::from(vehicle_identification_request);
                buffer[offset..].copy_from_slice(&bytes);

                buffer
            }
            DoipPayload::VehicleIdentificationRequestEid(vehicle_identification_request_eid) => {
                let bytes = <[u8; DOIP_COMMON_EID_LEN]>::from(vehicle_identification_request_eid);
                buffer[offset..].copy_from_slice(&bytes);

                buffer
            }
            DoipPayload::VehicleIdentificationRequestVin(vehicle_identification_request_vin) => {
                let bytes = <[u8; DOIP_COMMON_VIN_LEN]>::from(vehicle_identification_request_vin);
                buffer[offset..].copy_from_slice(&bytes);

                buffer
            }
            DoipPayload::VehicleAnnouncementMessage(vehicle_announcement_message) => {
                let bytes =
                    <[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG]>::from(vehicle_announcement_message);
                buffer[offset..].copy_from_slice(&bytes);

                buffer
            }
            DoipPayload::RoutingActivationRequest(routing_activation_request) => {
                let bytes =
                    <[u8; DOIP_ROUTING_ACTIVATION_REQ_LEN]>::from(routing_activation_request);
                buffer[offset..].copy_from_slice(&bytes);

                buffer
            }
            DoipPayload::RoutingActivationResponse(routing_activation_response) => {
                let bytes =
                    <[u8; DOIP_ROUTING_ACTIVATION_RES_LEN]>::from(routing_activation_response);
                buffer[offset..].copy_from_slice(&bytes);

                buffer
            }
            DoipPayload::AliveCheckRequest(alive_chec_request) => {
                let bytes = <[u8; 0]>::from(alive_chec_request);
                buffer[offset..].copy_from_slice(&bytes);

                buffer
            }
            DoipPayload::AliveCheckResponse(alive_chec_response) => {
                let bytes = <[u8; DOIP_DIAG_COMMON_SOURCE_LEN]>::from(alive_chec_response);
                buffer[offset..].copy_from_slice(&bytes);

                buffer
            }
            DoipPayload::EntityStatusRequest(entity_status_request) => {
                let bytes = <[u8; 0]>::from(entity_status_request);
                buffer[offset..].copy_from_slice(&bytes);

                buffer
            }
            DoipPayload::EntityStatusResponse(entity_status_response) => {
                let bytes = <[u8; DOIP_ENTITY_STATUS_RESPONSE_LEN]>::from(entity_status_response);
                buffer[offset..].copy_from_slice(&bytes);

                buffer
            }
            DoipPayload::PowerInformationRequest(power_information_request) => {
                let bytes = <[u8; 0]>::from(power_information_request);
                buffer[offset..].copy_from_slice(&bytes);

                buffer
            }
            DoipPayload::PowerInformationResponse(power_information_response) => {
                let bytes = <[u8; 1]>::from(power_information_response);
                buffer[offset..].copy_from_slice(&bytes);

                buffer
            }
            DoipPayload::DiagnosticMessage(diagnostic_message) => {
                let bytes = <[u8; N]>::from(diagnostic_message);
                buffer[offset..].copy_from_slice(&bytes);

                buffer
            }
            DoipPayload::DiagnosticMessageAck(diagnostic_message_ack) => {
                let bytes =
                    <[u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]>::from(
                        diagnostic_message_ack,
                    );
                buffer[offset..].copy_from_slice(&bytes);

                buffer
            }
            DoipPayload::DiagnosticMessageNack(diagnostic_message_nack) => {
                let bytes =
                    <[u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]>::from(
                        diagnostic_message_nack,
                    );
                buffer[offset..].copy_from_slice(&bytes);

                buffer
            }
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for DoipMessage<'a> {
    type Error = &'static str;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let (header_slice, rest) = value.split_at(DOIP_HEADER_LEN);

        let header_bytes: &[u8; DOIP_HEADER_LEN] = header_slice
            .try_into()
            .map_err(|_| "Invalid header length")?;

        let header: DoipHeader = DoipHeader::try_from(*header_bytes)?;

        let (payload_slice, _) = rest.split_at(header.payload_length as usize);

        let payload = DoipPayload::try_from((&header, payload_slice))?;

        Ok(DoipMessage { header, payload })
    }
}
