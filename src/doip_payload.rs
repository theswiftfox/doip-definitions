use alive_check_request::AliveCheckRequest;
use alive_check_response::AliveCheckResponse;
use diagnostic_message::DiagnosticMessage;
use diagnostic_message_ack::DiagnosticMessageAck;
use diagnostic_message_nack::DiagnosticMessageNack;
use entity_status_request::EntityStatusRequest;
use entity_status_response::EntityStatusResponse;
use generic_nack::GenericNack;
use power_information_request::PowerInformationRequest;
use power_information_response::PowerInformationResponse;
use routing_activation_request::RoutingActivationRequest;
use routing_activation_response::RoutingActivationResponse;
use vehicle_announcement_message::VehicleAnnouncementMessage;
use vehicle_identification_request::VehicleIdentificationRequest;
use vehicle_identification_request_eid::VehicleIdentificationRequestEid;
use vehicle_identification_request_vin::VehicleIdentificationRequestVin;

use crate::{
    definitions::{
        DOIP_COMMON_EID_LEN, DOIP_COMMON_VIN_LEN, DOIP_DIAG_COMMON_SOURCE_LEN,
        DOIP_DIAG_COMMON_TARGET_LEN, DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN,
        DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN, DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN,
        DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN, DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN,
        DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN, DOIP_ROUTING_ACTIVATION_RES_ISO_LEN,
        DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN, DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN,
    },
    header::{DoipHeader, PayloadType},
};

pub mod action_code;
pub mod activation_code;
pub mod activation_type;
pub mod diagnostic_ack;
pub mod diagnostic_nack;
pub mod nack_code;
pub mod node_type;
pub mod power_mode;
pub mod sync_status;

pub mod alive_check_request;
pub mod alive_check_response;
pub mod diagnostic_message;
pub mod diagnostic_message_ack;
pub mod diagnostic_message_nack;
pub mod entity_status_request;
pub mod entity_status_response;
pub mod generic_nack;
pub mod power_information_request;
pub mod power_information_response;
pub mod routing_activation_request;
pub mod routing_activation_response;
pub mod vehicle_announcement_message;
pub mod vehicle_identification_request;
pub mod vehicle_identification_request_eid;
pub mod vehicle_identification_request_vin;

/// Implemented across `DoIP` Payload Types for consistent encoding and decoding of buffers.
///
/// `DoipPayload` is implemented for all the `DoIP` Payload Types for the
/// purpose of consistent encoding and decoding as well as identification within
/// a buffer.
#[derive(Debug)]
pub enum DoipPayload<'a> {
    /// `GenericNack` variant to hold `GenericNack` struct
    GenericNack(GenericNack),

    /// `VehicleIdentificationRequest` variant to hold `VehicleIdentificationRequest` struct
    VehicleIdentificationRequest(VehicleIdentificationRequest),

    /// `VehicleIdentificationRequestEid` variant to hold `VehicleIdentificationRequestEid` struct
    VehicleIdentificationRequestEid(VehicleIdentificationRequestEid),

    /// `VehicleIdentificationRequestVin` variant to hold `VehicleIdentificationRequestVin` struct
    VehicleIdentificationRequestVin(VehicleIdentificationRequestVin),

    /// `VehicleAnnouncementMessage` variant to hold `VehicleAnnouncementMessage` struct
    VehicleAnnouncementMessage(VehicleAnnouncementMessage),

    /// `RoutingActivationRequest` variant to hold `RoutingActivationRequest` struct
    RoutingActivationRequest(RoutingActivationRequest),

    /// `RoutingActivationResponse` variant to hold `RoutingActivationResponse` struct
    RoutingActivationResponse(RoutingActivationResponse),

    /// `AliveCheckRequest` variant to hold `AliveCheckRequest` struct
    AliveCheckRequest(AliveCheckRequest),

    /// `AliveCheckResponse` variant to hold `AliveCheckResponse` struct
    AliveCheckResponse(AliveCheckResponse),

    /// `EntityStatusRequest` variant to hold `EntityStatusRequest` struct
    EntityStatusRequest(EntityStatusRequest),

    /// `EntityStatusResponse` variant to hold `EntityStatusResponse` struct
    EntityStatusResponse(EntityStatusResponse),

    /// `PowerInformationRequest` variant to hold `PowerInformationRequest` struct
    PowerInformationRequest(PowerInformationRequest),

    /// `PowerInformationResponse` variant to hold `PowerInformationResponse` struct
    PowerInformationResponse(PowerInformationResponse),

    /// `DiagnosticMessage` variant to hold `DiagnosticMessage` struct
    DiagnosticMessage(DiagnosticMessage<'a>),

    /// `DiagnosticMessageAck` variant to hold `DiagnosticMessageAck` struct
    DiagnosticMessageAck(DiagnosticMessageAck),

    /// `DiagnosticMessageNack` variant to hold `DiagnosticMessageNack` struct
    DiagnosticMessageNack(DiagnosticMessageNack),
}

impl<'a> TryFrom<(&DoipHeader, &'a [u8])> for DoipPayload<'a> {
    type Error = &'static str;

    fn try_from((header, slice): (&DoipHeader, &'a [u8])) -> Result<Self, Self::Error> {
        let payload = match header.payload_type {
            PayloadType::GenericNack => handle_generic_nack(slice),
            PayloadType::VehicleIdentificationRequest => {
                handle_vehicle_identification_request(slice)
            }
            PayloadType::VehicleIdentificationRequestEid => {
                handle_vehicle_identification_request_eid(slice)
            }
            PayloadType::VehicleIdentificationRequestVin => {
                handle_vehicle_identification_request_vin(slice)
            }
            PayloadType::VehicleAnnouncementMessage => handle_vehicle_announcement_message(slice),
            PayloadType::RoutingActivationRequest => handle_routing_activation_request(slice),
            PayloadType::RoutingActivationResponse => handle_routing_activation_response(slice),
            PayloadType::AliveCheckRequest => handle_alive_check_request(slice),
            PayloadType::AliveCheckResponse => handle_alive_check_response(slice),
            PayloadType::EntityStatusRequest => handle_entity_status_request(slice),
            PayloadType::EntityStatusResponse => handle_entity_status_response(slice),
            PayloadType::PowerInformationRequest => handle_power_information_request(slice),
            PayloadType::PowerInformationResponse => handle_power_information_response(slice),
            PayloadType::DiagnosticMessage => handle_diagnostic_message(slice),
            PayloadType::DiagnosticMessageAck => handle_diagnostic_message_ack(slice),
            PayloadType::DiagnosticMessageNack => handle_diagnostic_message_nack(slice),
        };

        payload
    }
}

fn handle_generic_nack(slice: &[u8]) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; 1] = slice
        .try_into()
        .map_err(|_| "Invalid GenericNack length.")?;
    Ok(DoipPayload::GenericNack(GenericNack::try_from(
        payload_bytes,
    )?))
}

fn handle_vehicle_identification_request(
    slice: &[u8],
) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; 0] = slice
        .try_into()
        .map_err(|_| "Invalid VehicleIdentificationRequest length.")?;
    Ok(DoipPayload::VehicleIdentificationRequest(
        VehicleIdentificationRequest::from(payload_bytes),
    ))
}

fn handle_vehicle_identification_request_eid(
    slice: &[u8],
) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; DOIP_COMMON_EID_LEN] = slice
        .try_into()
        .map_err(|_| "Invalid VehicleIdentificationRequestEid length.")?;
    Ok(DoipPayload::VehicleIdentificationRequestEid(
        VehicleIdentificationRequestEid::from(payload_bytes),
    ))
}

fn handle_vehicle_identification_request_vin(
    slice: &[u8],
) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; DOIP_COMMON_VIN_LEN] = slice
        .try_into()
        .map_err(|_| "Invalid VehicleIdentificationRequestVin length.")?;
    Ok(DoipPayload::VehicleIdentificationRequestVin(
        VehicleIdentificationRequestVin::from(payload_bytes),
    ))
}

fn handle_vehicle_announcement_message(slice: &[u8]) -> Result<DoipPayload<'static>, &'static str> {
    let vam_len = slice.len();
    if vam_len == 33 {
        let payload_bytes: [u8; DOIP_COMMON_VIN_LEN
            + DOIP_DIAG_COMMON_SOURCE_LEN
            + DOIP_COMMON_EID_LEN
            + DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN
            + 1
            + 1] = slice
            .try_into()
            .map_err(|_| "Invalid VehicleAnnouncementMessage length.")?;
        Ok(DoipPayload::VehicleAnnouncementMessage(
            VehicleAnnouncementMessage::try_from(payload_bytes)?,
        ))
    } else if vam_len == 32 {
        let payload_bytes: [u8; DOIP_COMMON_VIN_LEN
            + DOIP_DIAG_COMMON_SOURCE_LEN
            + DOIP_COMMON_EID_LEN
            + DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN
            + 1] = slice
            .try_into()
            .map_err(|_| "Invalid VehicleAnnouncementMessage length.")?;
        Ok(DoipPayload::VehicleAnnouncementMessage(
            VehicleAnnouncementMessage::try_from(payload_bytes)?,
        ))
    } else {
        Err("Invalid VehicleAnnouncementMessage length.")
    }
}

fn handle_routing_activation_request(slice: &[u8]) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN
        + 1
        + DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN] = slice
        .try_into()
        .map_err(|_| "Invalid RoutingActivationRequest length.")?;
    Ok(DoipPayload::RoutingActivationRequest(
        RoutingActivationRequest::try_from(payload_bytes)?,
    ))
}

fn handle_routing_activation_response(slice: &[u8]) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN
        + DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN
        + 1
        + DOIP_ROUTING_ACTIVATION_RES_ISO_LEN] = slice
        .try_into()
        .map_err(|_| "Invalid RoutingActivationResponse length.")?;
    Ok(DoipPayload::RoutingActivationResponse(
        RoutingActivationResponse::try_from(payload_bytes)?,
    ))
}

fn handle_alive_check_request(slice: &[u8]) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; 0] = slice
        .try_into()
        .map_err(|_| "Invalid AliveCheckRequest length.")?;
    Ok(DoipPayload::AliveCheckRequest(AliveCheckRequest::from(
        payload_bytes,
    )))
}

fn handle_alive_check_response(slice: &[u8]) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; DOIP_DIAG_COMMON_SOURCE_LEN] = slice
        .try_into()
        .map_err(|_| "Invalid AliveCheckResponse length.")?;
    Ok(DoipPayload::AliveCheckResponse(AliveCheckResponse::from(
        payload_bytes,
    )))
}

fn handle_entity_status_request(slice: &[u8]) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; 0] = slice
        .try_into()
        .map_err(|_| "Invalid EntityStatusRequest length.")?;
    Ok(DoipPayload::EntityStatusRequest(EntityStatusRequest::from(
        payload_bytes,
    )))
}

fn handle_entity_status_response(slice: &[u8]) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; 1
        + DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN
        + DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN
        + DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN] = slice
        .try_into()
        .map_err(|_| "Invalid EntityStatusResponse length.")?;
    Ok(DoipPayload::EntityStatusResponse(
        EntityStatusResponse::try_from(payload_bytes)?,
    ))
}

fn handle_power_information_request(slice: &[u8]) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; 0] = slice
        .try_into()
        .map_err(|_| "Invalid PowerInformationRequest length.")?;
    Ok(DoipPayload::PowerInformationRequest(
        PowerInformationRequest::from(payload_bytes),
    ))
}

fn handle_power_information_response(slice: &[u8]) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; 1] = slice
        .try_into()
        .map_err(|_| "Invalid PowerInformationResponse length.")?;
    Ok(DoipPayload::PowerInformationResponse(
        PowerInformationResponse::try_from(payload_bytes)?,
    ))
}

fn handle_diagnostic_message<'a>(slice: &'a [u8]) -> Result<DoipPayload<'a>, &'static str> {
    let diag_msg: DiagnosticMessage<'a> = DiagnosticMessage::try_from(slice)?;
    Ok(DoipPayload::DiagnosticMessage(diag_msg))
}

fn handle_diagnostic_message_ack(slice: &[u8]) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1] = slice
        .try_into()
        .map_err(|_| "Invalid DiagnosticMessageAck length.")?;
    Ok(DoipPayload::DiagnosticMessageAck(
        DiagnosticMessageAck::try_from(payload_bytes)?,
    ))
}

fn handle_diagnostic_message_nack(slice: &[u8]) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1] = slice
        .try_into()
        .map_err(|_| "Invalid DiagnosticMessageNack length.")?;
    Ok(DoipPayload::DiagnosticMessageNack(
        DiagnosticMessageNack::try_from(payload_bytes)?,
    ))
}
