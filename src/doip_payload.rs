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
        DOIP_DIAG_COMMON_TARGET_LEN, DOIP_ENTITY_STATUS_RESPONSE_LEN,
        DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN, DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN,
        DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN, DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN,
        DOIP_ROUTING_ACTIVATION_REQ_LEN, DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN,
        DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN, DOIP_ROUTING_ACTIVATION_RES_ISO_LEN,
        DOIP_ROUTING_ACTIVATION_RES_LEN, DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN,
        DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN, DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG,
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
#[derive(Debug, PartialEq)]
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
        if header.payload_length as usize != slice.len() {
            return Err("Slice does not match payload length.");
        }

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

impl<'a, const N: usize> TryFrom<DoipPayload<'a>> for [u8; N] {
    type Error = &'static str;

    fn try_from(payload: DoipPayload<'a>) -> Result<Self, Self::Error> {
        let mut buffer = [0u8; N];

        match payload {
            DoipPayload::GenericNack(gen_nack) => {
                let bytes = <[u8; 1]>::from(gen_nack);
                buffer.copy_from_slice(&bytes);
                Ok(buffer)
            }

            DoipPayload::VehicleIdentificationRequest(veh_id_req) => {
                let bytes = <[u8; 0]>::from(veh_id_req);
                buffer.copy_from_slice(&bytes);
                Ok(buffer)
            }

            DoipPayload::VehicleIdentificationRequestEid(veh_id_req_eid) => {
                let bytes = <[u8; DOIP_COMMON_EID_LEN]>::from(veh_id_req_eid);
                buffer.copy_from_slice(&bytes);
                Ok(buffer)
            }

            DoipPayload::VehicleIdentificationRequestVin(veh_id_req_vin) => {
                let bytes = <[u8; DOIP_COMMON_VIN_LEN]>::from(veh_id_req_vin);
                buffer.copy_from_slice(&bytes);
                Ok(buffer)
            }

            DoipPayload::VehicleAnnouncementMessage(veh_ann_msg) => {
                let bytes = <[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG]>::from(veh_ann_msg);
                buffer.copy_from_slice(&bytes);
                Ok(buffer)
            }

            DoipPayload::RoutingActivationRequest(routing_act_req) => {
                let bytes = <[u8; DOIP_ROUTING_ACTIVATION_REQ_LEN]>::from(routing_act_req);
                buffer.copy_from_slice(&bytes);
                Ok(buffer)
            }

            DoipPayload::RoutingActivationResponse(routing_act_res) => {
                let bytes = <[u8; DOIP_ROUTING_ACTIVATION_RES_LEN]>::from(routing_act_res);
                buffer.copy_from_slice(&bytes);
                Ok(buffer)
            }

            DoipPayload::AliveCheckRequest(alive_check_req) => {
                let bytes = <[u8; 0]>::from(alive_check_req);
                buffer.copy_from_slice(&bytes);
                Ok(buffer)
            }

            DoipPayload::AliveCheckResponse(alive_check_res) => {
                let bytes = <[u8; DOIP_DIAG_COMMON_SOURCE_LEN]>::from(alive_check_res);
                buffer.copy_from_slice(&bytes);
                Ok(buffer)
            }

            DoipPayload::EntityStatusRequest(ent_status_req) => {
                let bytes = <[u8; 0]>::from(ent_status_req);
                buffer.copy_from_slice(&bytes);
                Ok(buffer)
            }

            DoipPayload::EntityStatusResponse(ent_status_res) => {
                let bytes = <[u8; DOIP_ENTITY_STATUS_RESPONSE_LEN]>::from(ent_status_res);
                buffer.copy_from_slice(&bytes);
                Ok(buffer)
            }

            DoipPayload::PowerInformationRequest(power_info_req) => {
                let bytes = <[u8; 0]>::from(power_info_req);
                buffer.copy_from_slice(&bytes);
                Ok(buffer)
            }

            DoipPayload::PowerInformationResponse(power_info_res) => {
                let bytes = <[u8; 1]>::from(power_info_res);
                buffer.copy_from_slice(&bytes);
                Ok(buffer)
            }

            DoipPayload::DiagnosticMessage(diag_msg) => {
                let bytes = <[u8; N]>::try_from(diag_msg)?;
                buffer.copy_from_slice(&bytes);
                Ok(buffer)
            }

            DoipPayload::DiagnosticMessageAck(diag_msg_ack) => {
                let bytes =
                    <[u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]>::from(
                        diag_msg_ack,
                    );
                buffer.copy_from_slice(&bytes);
                Ok(buffer)
            }

            DoipPayload::DiagnosticMessageNack(diag_msg_nack) => {
                let bytes =
                    <[u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]>::from(
                        diag_msg_nack,
                    );
                buffer.copy_from_slice(&bytes);
                Ok(buffer)
            }
        }
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

#[cfg(test)]
mod tests {
    use crate::{
        definitions::{
            DOIP_ALIVE_CHECK_RESPONSE_SOURCE_LEN, DOIP_COMMON_EID_LEN, DOIP_COMMON_VIN_LEN,
            DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN,
            DOIP_ENTITY_STATUS_RESPONSE_LEN, DOIP_POWER_MODE_LEN, DOIP_ROUTING_ACTIVATION_REQ_LEN,
            DOIP_ROUTING_ACTIVATION_RES_LEN, DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN,
            DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG, DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT,
        },
        doip_payload::{
            handle_alive_check_request, handle_alive_check_response, handle_diagnostic_message,
            handle_diagnostic_message_ack, handle_diagnostic_message_nack,
            handle_entity_status_request, handle_entity_status_response, handle_generic_nack,
            handle_power_information_request, handle_power_information_response,
            handle_routing_activation_request, handle_routing_activation_response,
            handle_vehicle_announcement_message, handle_vehicle_identification_request,
            handle_vehicle_identification_request_eid, handle_vehicle_identification_request_vin,
        },
        header::{DoipHeader, PayloadType, ProtocolVersion},
        payload::{
            ActionCode, ActivationCode, ActivationType, AliveCheckRequest, AliveCheckResponse,
            DiagnosticAckCode, DiagnosticMessage, DiagnosticMessageAck, DiagnosticMessageNack,
            DiagnosticNackCode, EntityStatusRequest, EntityStatusResponse, GenericNack, NackCode,
            NodeType, PowerInformationRequest, PowerInformationResponse, PowerMode,
            RoutingActivationRequest, RoutingActivationResponse, SyncStatus,
            VehicleAnnouncementMessage, VehicleIdentificationRequest,
            VehicleIdentificationRequestEid, VehicleIdentificationRequestVin,
        },
    };

    use super::DoipPayload;

    #[test]
    fn test_try_from_bytes_generic_nack() {
        let header = DoipHeader {
            protocol_version: ProtocolVersion::Iso13400_2012,
            inverse_protocol_version: 0xfd,
            payload_type: PayloadType::GenericNack,
            payload_length: 0x00000001,
        };

        let payload = GenericNack {
            nack_code: NackCode::IncorrectPatternFormat,
        };

        let payload_bytes: &[u8] = &<[u8; 1]>::from(payload);

        let bytes = DoipPayload::try_from((&header, payload_bytes));

        assert_eq!(bytes.unwrap(), DoipPayload::GenericNack(payload))
    }

    #[test]
    fn test_try_from_bytes_vehicle_id_req() {
        let payload = VehicleIdentificationRequest {};

        let payload_bytes: &[u8] = &<[u8; 0]>::from(payload);

        let header = DoipHeader {
            protocol_version: ProtocolVersion::Iso13400_2012,
            inverse_protocol_version: 0xfd,
            payload_type: PayloadType::VehicleIdentificationRequest,
            payload_length: payload_bytes.len() as u32,
        };

        let bytes = DoipPayload::try_from((&header, payload_bytes));

        assert_eq!(
            bytes.unwrap(),
            DoipPayload::VehicleIdentificationRequest(payload)
        )
    }

    #[test]
    fn test_try_from_bytes_vehicle_id_req_eid() {
        let payload = VehicleIdentificationRequestEid {
            eid: [0u8; DOIP_COMMON_EID_LEN],
        };

        let payload_bytes: &[u8] = &<[u8; DOIP_COMMON_EID_LEN]>::from(payload);

        let header = DoipHeader {
            protocol_version: ProtocolVersion::Iso13400_2012,
            inverse_protocol_version: 0xfd,
            payload_type: PayloadType::VehicleIdentificationRequestEid,
            payload_length: payload_bytes.len() as u32,
        };

        let bytes = DoipPayload::try_from((&header, payload_bytes));

        assert_eq!(
            bytes.unwrap(),
            DoipPayload::VehicleIdentificationRequestEid(payload)
        )
    }

    #[test]
    fn test_try_from_bytes_vehicle_id_req_vin() {
        let payload = VehicleIdentificationRequestVin {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
        };

        let payload_bytes: &[u8] = &<[u8; DOIP_COMMON_VIN_LEN]>::from(payload);

        let header = DoipHeader {
            protocol_version: ProtocolVersion::Iso13400_2012,
            inverse_protocol_version: 0xfd,
            payload_type: PayloadType::VehicleIdentificationRequestVin,
            payload_length: payload_bytes.len() as u32,
        };

        let bytes = DoipPayload::try_from((&header, payload_bytes));

        assert_eq!(
            bytes.unwrap(),
            DoipPayload::VehicleIdentificationRequestVin(payload)
        )
    }

    #[test]
    fn test_try_from_bytes_vehicle_announce_msg_short() {
        let payload = VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x01, 0x02],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: None,
        };

        let payload_bytes: &[u8] = &<[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT]>::from(payload);

        let header = DoipHeader {
            protocol_version: ProtocolVersion::Iso13400_2012,
            inverse_protocol_version: 0xfd,
            payload_type: PayloadType::VehicleAnnouncementMessage,
            payload_length: payload_bytes.len() as u32,
        };

        let bytes = DoipPayload::try_from((&header, payload_bytes));

        assert_eq!(
            bytes.unwrap(),
            DoipPayload::VehicleAnnouncementMessage(payload)
        )
    }

    #[test]
    fn test_try_from_bytes_vehicle_announce_msg_long() {
        let payload = VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x01, 0x02],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: Some(SyncStatus::VinGidSynchronized),
        };

        let payload_bytes: &[u8] = &<[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG]>::from(payload);

        let header = DoipHeader {
            protocol_version: ProtocolVersion::Iso13400_2012,
            inverse_protocol_version: 0xfd,
            payload_type: PayloadType::VehicleAnnouncementMessage,
            payload_length: payload_bytes.len() as u32,
        };

        let bytes = DoipPayload::try_from((&header, payload_bytes));

        assert_eq!(
            bytes.unwrap(),
            DoipPayload::VehicleAnnouncementMessage(payload)
        )
    }

    #[test]
    fn test_try_from_bytes_routing_act_req() {
        let payload = RoutingActivationRequest {
            source_address: [0x01, 0x02],
            activation_type: ActivationType::Default,
            buffer: [0x01, 0x02, 0x03, 0x04],
        };

        let payload_bytes: &[u8] = &<[u8; DOIP_ROUTING_ACTIVATION_REQ_LEN]>::from(payload);

        let header = DoipHeader {
            protocol_version: ProtocolVersion::Iso13400_2012,
            inverse_protocol_version: 0xfd,
            payload_type: PayloadType::RoutingActivationRequest,
            payload_length: payload_bytes.len() as u32,
        };

        let bytes = DoipPayload::try_from((&header, payload_bytes));

        assert_eq!(
            bytes.unwrap(),
            DoipPayload::RoutingActivationRequest(payload)
        )
    }

    #[test]
    fn test_handle_generic_nack_pass() {
        let payload = &[0x00];
        let res = handle_generic_nack(payload);

        assert!(res.is_ok())
    }

    #[test]
    fn test_handle_generic_nack_invalid_length() {
        let payload = &[];
        let res = handle_generic_nack(payload);

        assert!(res.is_err())
    }

    #[test]
    fn test_handle_vehicle_identification_request_pass() {
        let payload = &[];
        let res = handle_vehicle_identification_request(payload);

        assert!(res.is_ok())
    }

    #[test]
    fn test_handle_vehicle_identification_request_invalid_length() {
        let payload = &[0x00];
        let res = handle_vehicle_identification_request(payload);

        assert!(res.is_err())
    }

    #[test]
    fn test_handle_vehicle_identification_request_eid_pass() {
        let payload = &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let res = handle_vehicle_identification_request_eid(payload);

        assert!(res.is_ok())
    }

    #[test]
    fn test_handle_vehicle_identification_request_eid_invalid_length() {
        let payload = &[0x00];
        let res = handle_vehicle_identification_request_eid(payload);

        assert!(res.is_err())
    }

    #[test]
    fn test_handle_vehicle_identification_request_vin_pass() {
        let payload = &[0u8; DOIP_COMMON_VIN_LEN];
        let res = handle_vehicle_identification_request_vin(payload);

        assert!(res.is_ok())
    }

    #[test]
    fn test_handle_vehicle_identification_request_vin_invalid_length() {
        let payload = &[0x00];
        let res = handle_vehicle_identification_request_vin(payload);

        assert!(res.is_err())
    }

    #[test]
    fn test_handle_vehicle_announcement_message_long_pass() {
        let payload = &[0u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG];
        let res = handle_vehicle_announcement_message(payload);

        assert!(res.is_ok())
    }

    #[test]
    fn test_handle_vehicle_announcement_message_long_invalid_length() {
        let payload = &[0x00];
        let res = handle_vehicle_announcement_message(payload);

        assert!(res.is_err())
    }

    #[test]
    fn test_handle_vehicle_announcement_message_short_pass() {
        let payload = &[0u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT];
        let res = handle_vehicle_announcement_message(payload);

        assert!(res.is_ok())
    }

    #[test]
    fn test_handle_vehicle_announcement_message_short_invalid_length() {
        let payload = &[0x00];
        let res = handle_vehicle_announcement_message(payload);

        assert!(res.is_err())
    }

    #[test]
    fn test_handle_routing_activation_request_pass() {
        let payload: [u8; DOIP_ROUTING_ACTIVATION_REQ_LEN] =
            <[u8; DOIP_ROUTING_ACTIVATION_REQ_LEN]>::from(RoutingActivationRequest {
                source_address: [0x01, 0x02],
                activation_type: ActivationType::Default,
                buffer: [0x00, 0x00, 0x00, 0x00],
            });

        let result = handle_routing_activation_request(&payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_routing_activation_request_invalid_length() {
        let payload = &[0x00];
        let res = handle_routing_activation_request(payload);

        assert!(res.is_err())
    }

    #[test]
    fn test_handle_routing_activation_response_pass() {
        let payload: [u8; DOIP_ROUTING_ACTIVATION_RES_LEN] =
            <[u8; DOIP_ROUTING_ACTIVATION_RES_LEN]>::from(RoutingActivationResponse {
                logical_address: [0xff, 0xff],
                source_address: [0xff, 0xff],
                activation_code: ActivationCode::ActivatedConfirmationRequired,
                buffer: [0x00, 0x00, 0x00, 0x00],
            });

        let result = handle_routing_activation_response(&payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_routing_activation_response_invalid_length() {
        let payload = &[0x00];
        let res = handle_routing_activation_response(payload);

        assert!(res.is_err())
    }

    #[test]
    fn test_handle_alive_check_request_pass() {
        let payload: [u8; 0] = <[u8; 0]>::from(AliveCheckRequest {});

        let result = handle_alive_check_request(&payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_alive_check_request_invalid_length() {
        let payload = &[0x00];
        let res = handle_alive_check_request(payload);

        assert!(res.is_err())
    }

    #[test]
    fn test_handle_alive_check_response_pass() {
        let payload: [u8; DOIP_ALIVE_CHECK_RESPONSE_SOURCE_LEN] =
            <[u8; DOIP_ALIVE_CHECK_RESPONSE_SOURCE_LEN]>::from(AliveCheckResponse {
                source_address: [0x01, 0x02],
            });

        let result = handle_alive_check_response(&payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_alive_check_response_invalid_length() {
        let payload = &[0x00];
        let res = handle_alive_check_response(payload);

        assert!(res.is_err())
    }

    #[test]
    fn test_handle_entity_status_request_pass() {
        let payload: [u8; 0] = <[u8; 0]>::from(EntityStatusRequest {});

        let result = handle_entity_status_request(&payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_entity_status_request_invalid_length() {
        let payload = &[0x00];
        let res = handle_entity_status_request(payload);

        assert!(res.is_err())
    }

    #[test]
    fn test_handle_entity_status_response_pass() {
        let payload: [u8; DOIP_ENTITY_STATUS_RESPONSE_LEN] =
            <[u8; DOIP_ENTITY_STATUS_RESPONSE_LEN]>::from(EntityStatusResponse {
                node_type: NodeType::DoipGateway,
                max_concurrent_sockets: [0x01],
                currently_open_sockets: [0x02],
                max_data_size: [0x03, 0x04, 0x05, 0x06],
            });

        let result = handle_entity_status_response(&payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_entity_status_response_invalid_length() {
        let payload = &[0x00];
        let res = handle_entity_status_response(payload);

        assert!(res.is_err())
    }

    #[test]
    fn test_handle_power_information_request_pass() {
        let payload: [u8; 0] = <[u8; 0]>::from(PowerInformationRequest {});

        let result = handle_power_information_request(&payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_power_information_request_invalid_length() {
        let payload = &[0x00];
        let res = handle_power_information_request(payload);

        assert!(res.is_err())
    }

    #[test]
    fn test_handle_power_information_response_pass() {
        let payload: [u8; DOIP_POWER_MODE_LEN] =
            <[u8; DOIP_POWER_MODE_LEN]>::from(PowerInformationResponse {
                power_mode: PowerMode::Ready,
            });

        let result = handle_power_information_response(&payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_power_information_response_invalid_length() {
        let payload = &[];
        let res = handle_power_information_response(payload);

        assert!(res.is_err())
    }

    #[test]
    fn test_handle_diagnostic_message_pass() {
        const N: usize = 5;
        let payload = <[u8; N]>::try_from(DiagnosticMessage {
            source_address: [0x01, 0x02],
            target_address: [0x01, 0x02],
            message: &[0x05],
        })
        .unwrap();

        let result = handle_diagnostic_message(&payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_diagnostic_message_invalid_length() {
        let payload = &[];
        let res = handle_diagnostic_message(payload);

        assert!(res.is_err())
    }

    #[test]
    fn test_handle_diagnostic_message_ack_pass() {
        let payload =
            <[u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]>::try_from(
                DiagnosticMessageAck {
                    source_address: [0x01, 0x02],
                    target_address: [0x01, 0x02],
                    ack_code: DiagnosticAckCode::Acknowledged,
                },
            )
            .unwrap();

        let result = handle_diagnostic_message_ack(&payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_diagnostic_message_ack_invalid_length() {
        let payload = &[];
        let res = handle_diagnostic_message_ack(payload);

        assert!(res.is_err())
    }

    #[test]
    fn test_handle_diagnostic_message_nack_pass() {
        let payload =
            <[u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]>::try_from(
                DiagnosticMessageNack {
                    source_address: [0x01, 0x02],
                    target_address: [0x01, 0x02],
                    nack_code: DiagnosticNackCode::OutOfMemory,
                },
            )
            .unwrap();

        let result = handle_diagnostic_message_nack(&payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_diagnostic_message_nack_invalid_length() {
        let payload = &[];
        let res = handle_diagnostic_message_nack(payload);

        assert!(res.is_err())
    }
}
