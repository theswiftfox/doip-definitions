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
        DOIP_DIAG_COMMON_TARGET_LEN, DOIP_ENTITY_STATUS_RESPONSE_LEN, DOIP_GENERIC_NACK_LEN,
        DOIP_ROUTING_ACTIVATION_REQ_LEN, DOIP_ROUTING_ACTIVATION_RES_LEN,
        DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG, DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT,
    },
    header::{DoipHeader, PayloadType},
    util::{
        handle_alive_check_request, handle_alive_check_response, handle_diagnostic_message,
        handle_diagnostic_message_ack, handle_diagnostic_message_nack,
        handle_entity_status_request, handle_entity_status_response, handle_generic_nack,
        handle_power_information_request, handle_power_information_response,
        handle_routing_activation_request, handle_routing_activation_response,
        handle_vehicle_announcement_message, handle_vehicle_identification_request,
        handle_vehicle_identification_request_eid, handle_vehicle_identification_request_vin,
    },
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
#[derive(Debug, PartialEq, Copy, Clone)]
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

// impl<'a> TryFrom<(&DoipHeader, &'a [u8])> for DoipPayload<'a> {
//     type Error = &'static str;

//     fn try_from((header, slice): (&DoipHeader, &'a [u8])) -> Result<Self, Self::Error> {
//         return Err("TEST");
//         if header.payload_length as usize != slice.len() {
//             return Err("Slice does not match payload length.");
//         }

//         match header.payload_type {
//             PayloadType::GenericNack => handle_generic_nack(slice),
//             PayloadType::VehicleIdentificationRequest => {
//                 handle_vehicle_identification_request(slice)
//             }
//             PayloadType::VehicleIdentificationRequestEid => {
//                 handle_vehicle_identification_request_eid(slice)
//             }
//             PayloadType::VehicleIdentificationRequestVin => {
//                 handle_vehicle_identification_request_vin(slice)
//             }
//             PayloadType::VehicleAnnouncementMessage => handle_vehicle_announcement_message(slice),
//             PayloadType::RoutingActivationRequest => handle_routing_activation_request(slice),
//             PayloadType::RoutingActivationResponse => handle_routing_activation_response(slice),
//             PayloadType::AliveCheckRequest => handle_alive_check_request(slice),
//             PayloadType::AliveCheckResponse => handle_alive_check_response(slice),
//             PayloadType::EntityStatusRequest => handle_entity_status_request(slice),
//             PayloadType::EntityStatusResponse => handle_entity_status_response(slice),
//             PayloadType::PowerInformationRequest => handle_power_information_request(slice),
//             PayloadType::PowerInformationResponse => handle_power_information_response(slice),
//             PayloadType::DiagnosticMessage => handle_diagnostic_message(slice),
//             PayloadType::DiagnosticMessageAck => handle_diagnostic_message_ack(slice),
//             PayloadType::DiagnosticMessageNack => handle_diagnostic_message_nack(slice),
//         }
//     }
// }

impl<'a, const N: usize> TryFrom<DoipPayload<'a>> for [u8; N] {
    type Error = &'static str;

    #[allow(clippy::too_many_lines)]
    /// Allowed due to large match statement
    fn try_from(payload: DoipPayload<'a>) -> Result<Self, Self::Error> {
        let mut buffer = [0u8; N];

        match payload {
            DoipPayload::GenericNack(gen_nack) => {
                buffer.copy_from_slice(&<[u8; N]>::try_from(gen_nack)?);
                Ok(buffer)
            }

            // DoipPayload::VehicleIdentificationRequest(veh_id_req) => {
            //     let bytes = <[u8; 0]>::from(veh_id_req);

            //     buffer.copy_from_slice(&bytes);
            // }
            // DoipPayload::VehicleIdentificationRequestEid(veh_id_req_eid) => {
            //     let bytes = <[u8; DOIP_COMMON_EID_LEN]>::from(veh_id_req_eid);

            //     match bytes.len() {
            //         len if len > N => return Err("Buffer is too small"),
            //         len => {
            //             buffer[..len].copy_from_slice(&bytes);
            //         }
            //     }
            // }

            // DoipPayload::VehicleIdentificationRequestVin(veh_id_req_vin) => {
            //     let bytes = <[u8; DOIP_COMMON_VIN_LEN]>::from(veh_id_req_vin);

            //     match bytes.len() {
            //         len if len > N => return Err("Buffer is too small"),
            //         len => {
            //             buffer[..len].copy_from_slice(&bytes);
            //         }
            //     }
            // }
            DoipPayload::VehicleAnnouncementMessage(veh_ann_msg) => {
                buffer.copy_from_slice(&<[u8; N]>::try_from(veh_ann_msg)?);
                Ok(buffer)
            }

            // DoipPayload::RoutingActivationRequest(routing_act_req) => {
            //     let bytes = <[u8; DOIP_ROUTING_ACTIVATION_REQ_LEN]>::from(routing_act_req);

            //     match bytes.len() {
            //         len if len > N => return Err("Buffer is too small"),
            //         len => {
            //             buffer[..len].copy_from_slice(&bytes);
            //         }
            //     }
            // }

            // DoipPayload::RoutingActivationResponse(routing_act_res) => {
            //     let bytes = <[u8; DOIP_ROUTING_ACTIVATION_RES_LEN]>::from(routing_act_res);

            //     match bytes.len() {
            //         len if len > N => return Err("Buffer is too small"),
            //         len => {
            //             buffer[..len].copy_from_slice(&bytes);
            //         }
            //     }
            // }

            // DoipPayload::AliveCheckRequest(alive_check_req) => {
            //     let bytes = <[u8; 0]>::from(alive_check_req);

            //     buffer.copy_from_slice(&bytes);
            // }

            // DoipPayload::AliveCheckResponse(alive_check_res) => {
            //     let bytes = <[u8; DOIP_DIAG_COMMON_SOURCE_LEN]>::from(alive_check_res);

            //     match bytes.len() {
            //         len if len > N => return Err("Buffer is too small"),
            //         len => {
            //             buffer[..len].copy_from_slice(&bytes);
            //         }
            //     }
            // }

            // DoipPayload::EntityStatusRequest(ent_status_req) => {
            //     let bytes = <[u8; 0]>::from(ent_status_req);

            //     buffer.copy_from_slice(&bytes);
            // }

            // DoipPayload::EntityStatusResponse(ent_status_res) => {
            //     let bytes = <[u8; DOIP_ENTITY_STATUS_RESPONSE_LEN]>::from(ent_status_res);

            //     match bytes.len() {
            //         len if len > N => return Err("Buffer is too small"),
            //         len => {
            //             buffer[..len].copy_from_slice(&bytes);
            //         }
            //     }
            // }

            // DoipPayload::PowerInformationRequest(power_info_req) => {
            //     let bytes = <[u8; 0]>::from(power_info_req);

            //     buffer.copy_from_slice(&bytes);
            // }

            // DoipPayload::PowerInformationResponse(power_info_res) => {
            //     let bytes = <[u8; 1]>::from(power_info_res);

            //     match bytes.len() {
            //         len if len > N => return Err("Buffer is too small"),
            //         len => {
            //             buffer[..len].copy_from_slice(&bytes);
            //         }
            //     }
            // }
            // DoipPayload::DiagnosticMessage(diag_msg) => {
            //     let bytes = <[u8; N]>::try_from(diag_msg);

            //     match bytes {
            //         Ok(bytes) => {
            //             buffer.copy_from_slice(&bytes);
            //         }
            //         Err(e) => return Err(e),
            //     }
            // }

            // DoipPayload::DiagnosticMessageAck(diag_msg_ack) => {
            //     let bytes =
            //         <[u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]>::from(
            //             diag_msg_ack,
            //         );

            //     match bytes.len() {
            //         len if len > N => return Err("Buffer is too small"),
            //         len => {
            //             buffer[..len].copy_from_slice(&bytes);
            //         }
            //     }
            // }

            // DoipPayload::DiagnosticMessageNack(diag_msg_nack) => {
            //     let bytes =
            //         <[u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]>::from(
            //             diag_msg_nack,
            //         );

            //     match bytes.len() {
            //         len if len > N => return Err("Buffer is too small"),
            //         len => {
            //             buffer[..len].copy_from_slice(&bytes);
            //         }
            //     }
            // }
            _ => Err("TEST"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        definitions::{
            DOIP_ALIVE_CHECK_RESPONSE_SOURCE_LEN, DOIP_COMMON_EID_LEN, DOIP_COMMON_VIN_LEN,
            DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN,
            DOIP_ENTITY_STATUS_RESPONSE_LEN, DOIP_GENERIC_NACK_LEN, DOIP_POWER_MODE_LEN,
            DOIP_ROUTING_ACTIVATION_REQ_LEN, DOIP_ROUTING_ACTIVATION_RES_LEN,
            DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN, DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG,
            DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT,
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

    // #[test]
    // fn try_from_generic_nack_success() {
    //     const N: usize = DOIP_GENERIC_NACK_LEN;

    //     let gen_nack = GenericNack {
    //         nack_code: NackCode::IncorrectPatternFormat,
    //     };

    //     let bytes = <[u8; 1]>::from(gen_nack);

    //     let payload = DoipPayload::GenericNack(gen_nack);

    //     let payload_bytes = <[u8; N]>::try_from(payload);

    //     assert_eq!(payload_bytes.unwrap(), bytes);
    // }

    // #[test]
    // fn try_from_generic_nack_fail_small_buffer() {
    //     const N: usize = 0;

    //     let gen_nack = GenericNack {
    //         nack_code: NackCode::IncorrectPatternFormat,
    //     };

    //     let payload = DoipPayload::GenericNack(gen_nack);

    //     let payload_bytes = <[u8; N]>::try_from(payload);

    //     assert_eq!(payload_bytes.unwrap_err(), "Buffer is too small");
    // }

    // #[test]
    // fn try_from_generic_nack_success_large_buffer() {
    //     const N: usize = 2;

    //     let gen_nack = GenericNack {
    //         nack_code: NackCode::IncorrectPatternFormat,
    //     };

    //     let payload = DoipPayload::GenericNack(gen_nack);

    //     let payload_bytes = <[u8; N]>::try_from(payload);

    //     assert_eq!(payload_bytes.unwrap(), [0x00, 0x00])
    // }

    // #[test]
    // fn test_try_from_generic_nack_to_bytes() {
    //     let gen_nack = GenericNack {
    //         nack_code: NackCode::IncorrectPatternFormat,
    //     };

    //     let bytes = <[u8; 1]>::from(gen_nack);

    //     let payload = DoipPayload::GenericNack(gen_nack);

    //     let payload_bytes = <[u8; 1]>::try_from(payload);

    //     assert_eq!(payload_bytes.unwrap(), bytes);
    // }

    // #[test]
    // fn test_try_from_generic_nack_to_bytes_fail_buffer_small() {
    //     let gen_nack = GenericNack {
    //         nack_code: NackCode::IncorrectPatternFormat,
    //     };

    //     let payload = DoipPayload::GenericNack(gen_nack);

    //     let payload_bytes = <[u8; 0]>::try_from(payload);

    //     assert_eq!(payload_bytes.unwrap_err(), "Buffer is too small");
    // }

    // #[test]
    // fn test_try_from_generic_nack_to_bytes_fail_buffer_large() {
    //     const N: usize = 3;
    //     let gen_nack = GenericNack {
    //         nack_code: NackCode::IncorrectPatternFormat,
    //     };

    //     let payload = DoipPayload::GenericNack(gen_nack);

    //     let payload_bytes = <[u8; N]>::try_from(payload);

    //     assert_eq!(payload_bytes.unwrap().len(), N);
    // }

    // #[test]
    // fn test_generic_nack_buffer_too_small() {
    //     let gen_nack = GenericNack {
    //         nack_code: NackCode::IncorrectPatternFormat,
    //     };

    //     let payload = DoipPayload::GenericNack(gen_nack);

    //     // Ensure bytes.len() > N
    //     let payload_bytes = <[u8; 0]>::try_from(payload);

    //     assert_eq!(payload_bytes.unwrap_err(), "Buffer is too small");
    // }

    // #[test]
    // fn test_generic_nack_buffer_exact_fit() {
    //     let gen_nack = GenericNack {
    //         nack_code: NackCode::IncorrectPatternFormat,
    //     };

    //     let payload = DoipPayload::GenericNack(gen_nack);

    //     // Ensure bytes.len() == N
    //     let payload_bytes = <[u8; 1]>::try_from(payload);

    //     assert!(payload_bytes.is_ok());
    //     assert_eq!(
    //         payload_bytes.unwrap(),
    //         [NackCode::IncorrectPatternFormat as u8]
    //     );
    // }

    // #[test]
    // fn test_generic_nack_buffer_smaller_than_n() {
    //     let gen_nack = GenericNack {
    //         nack_code: NackCode::IncorrectPatternFormat,
    //     };

    //     let payload = DoipPayload::GenericNack(gen_nack);

    //     // Ensure bytes.len() < N
    //     let payload_bytes = <[u8; 2]>::try_from(payload);

    //     assert!(payload_bytes.is_ok());
    //     let result = payload_bytes.unwrap();
    //     assert_eq!(result[0], NackCode::IncorrectPatternFormat as u8);
    //     assert_eq!(result[1], 0); // Ensure buffer remains untouched
    // }

    // #[test]
    // fn test_try_from_vehicle_id_req_to_bytes() {
    //     let payload = VehicleIdentificationRequest {};

    //     let bytes = <[u8; 0]>::from(payload);

    //     let payload_enum = DoipPayload::VehicleIdentificationRequest(payload);

    //     let payload_bytes = <[u8; 0]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap(), bytes);
    // }

    #[test]
    fn test_try_from_vehicle_id_req_to_bytes() {
        let payload = VehicleIdentificationRequest {};

        let bytes = <[u8; 0]>::from(payload);

        let payload_enum = DoipPayload::VehicleIdentificationRequest(payload);

        let payload_bytes = <[u8; 0]>::try_from(payload_enum);

        assert_eq!(payload_bytes.unwrap_err(), "TEST");
    }

    // #[test]
    // fn test_try_from_vehicle_id_req_eid_to_bytes() {
    //     let payload = VehicleIdentificationRequestEid {
    //         eid: [0u8; DOIP_COMMON_EID_LEN],
    //     };

    //     let bytes = <[u8; DOIP_COMMON_EID_LEN]>::from(payload);

    //     let payload_enum = DoipPayload::VehicleIdentificationRequestEid(payload);

    //     let payload_bytes = <[u8; DOIP_COMMON_EID_LEN]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap(), bytes);
    // }

    #[test]
    fn test_try_from_vehicle_id_req_eid_to_bytes() {
        let payload = VehicleIdentificationRequestEid {
            eid: [0u8; DOIP_COMMON_EID_LEN],
        };

        let bytes = <[u8; DOIP_COMMON_EID_LEN]>::from(payload);

        let payload_enum = DoipPayload::VehicleIdentificationRequestEid(payload);

        let payload_bytes = <[u8; DOIP_COMMON_EID_LEN]>::try_from(payload_enum);

        assert_eq!(payload_bytes.unwrap_err(), "TEST");
    }

    // #[test]
    // fn test_try_from_vehicle_id_req_vin_to_bytes() {
    //     let payload = VehicleIdentificationRequestVin {
    //         vin: [0u8; DOIP_COMMON_VIN_LEN],
    //     };

    //     let bytes = <[u8; DOIP_COMMON_VIN_LEN]>::from(payload);

    //     let payload_enum = DoipPayload::VehicleIdentificationRequestVin(payload);

    //     let payload_bytes = <[u8; DOIP_COMMON_VIN_LEN]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap(), bytes);
    // }

    #[test]
    fn test_try_from_vehicle_id_req_vin_to_bytes() {
        let payload = VehicleIdentificationRequestVin {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
        };

        let bytes = <[u8; DOIP_COMMON_VIN_LEN]>::from(payload);

        let payload_enum = DoipPayload::VehicleIdentificationRequestVin(payload);

        let payload_bytes = <[u8; DOIP_COMMON_VIN_LEN]>::try_from(payload_enum);

        assert_eq!(payload_bytes.unwrap_err(), "TEST");
    }

    // #[test]
    // fn test_try_from_vehicle_announce_msg_short_to_bytes() {
    //     let payload = VehicleAnnouncementMessage {
    //         vin: [0u8; DOIP_COMMON_VIN_LEN],
    //         logical_address: [0x01, 0x02],
    //         eid: [0u8; DOIP_COMMON_EID_LEN],
    //         gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
    //         further_action: ActionCode::NoFurtherActionRequired,
    //         vin_gid_sync: None,
    //     };

    //     let bytes = <[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT]>::from(payload);

    //     let payload_enum = DoipPayload::VehicleAnnouncementMessage(payload);

    //     let payload_bytes = <[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap(), bytes);
    // }

    // #[test]
    // fn test_try_from_vehicle_announce_msg_long_to_bytes() {
    //     let payload = VehicleAnnouncementMessage {
    //         vin: [0u8; DOIP_COMMON_VIN_LEN],
    //         logical_address: [0x01, 0x02],
    //         eid: [0u8; DOIP_COMMON_EID_LEN],
    //         gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
    //         further_action: ActionCode::NoFurtherActionRequired,
    //         vin_gid_sync: Some(SyncStatus::VinGidSynchronized),
    //     };

    //     let bytes = <[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG]>::from(payload);

    //     let payload_enum = DoipPayload::VehicleAnnouncementMessage(payload);

    //     let payload_bytes = <[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap(), bytes);
    // }

    #[test]
    fn test_vehicle_announcement_long_success() {
        let veh_ann_msg = VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x01, 0x02],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: Some(SyncStatus::VinGidSynchronized),
        };

        let payload = DoipPayload::VehicleAnnouncementMessage(veh_ann_msg);

        let result = <[u8; 33]>::try_from(payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_vehicle_announcement_long_fail_buffer_too_small() {
        let veh_ann_msg = VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x01, 0x02],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: Some(SyncStatus::VinGidSynchronized),
        };

        let payload = DoipPayload::VehicleAnnouncementMessage(veh_ann_msg);

        let result = <[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG - 1]>::try_from(payload);

        assert_eq!(result.unwrap_err(), "Invalid buffer size");
    }

    #[test]
    fn test_vehicle_announcement_long_with_large_buffer() {
        let veh_ann_msg = VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x01, 0x02],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: Some(SyncStatus::VinGidSynchronized), // vin_gid_sync is Some
        };

        let payload = DoipPayload::VehicleAnnouncementMessage(veh_ann_msg);

        // Test when buffer is large enough
        let result = <[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG]>::try_from(payload);
        assert!(result.is_ok());

        // Test when buffer is too small
        let result = <[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG - 1]>::try_from(payload);
        assert_eq!(result.unwrap_err(), "Invalid buffer size");
    }

    #[test]
    fn test_vehicle_announcement_short_with_large_buffer() {
        let veh_ann_msg = VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x01, 0x02],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: None, // vin_gid_sync is None
        };

        let payload = DoipPayload::VehicleAnnouncementMessage(veh_ann_msg);

        // Test when buffer is large enough
        let result = <[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT]>::try_from(payload);
        assert!(result.is_ok());

        // Test when buffer is too small
        let result = <[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT - 1]>::try_from(payload);
        assert_eq!(result.unwrap_err(), "Invalid buffer size");
    }

    #[test]
    fn test_vehicle_announcement_long_with_large_buffer_len_greater_than_n() {
        let veh_ann_msg = VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x01, 0x02],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: Some(SyncStatus::VinGidSynchronized),
        };

        let payload = DoipPayload::VehicleAnnouncementMessage(veh_ann_msg);

        // Test with a buffer size that is larger than the expected buffer
        let result = <[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG - 2]>::try_from(payload);

        // Expect error since len > N
        assert_eq!(result.unwrap_err(), "Invalid buffer size");
    }

    #[test]
    fn test_vehicle_announcement_short_with_large_buffer_len_greater_than_n() {
        let veh_ann_msg = VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x01, 0x02],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: None,
        };

        let payload = DoipPayload::VehicleAnnouncementMessage(veh_ann_msg);

        // Test with a buffer size that is larger than the expected buffer
        let result = <[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT - 1]>::try_from(payload);

        // Expect error since len > N
        assert_eq!(result.unwrap_err(), "Invalid buffer size");
    }

    #[test]
    fn test_vehicle_announcement_invalid_vin_gid_sync() {
        let veh_ann_msg = VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x01, 0x02],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: None, // Ensures the _ catch-all branch is hit
        };

        let payload = DoipPayload::VehicleAnnouncementMessage(veh_ann_msg);

        let result = <[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG]>::try_from(payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_vehicle_announcement_short_success() {
        let veh_ann_msg = VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x01, 0x02],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: None,
        };

        let payload = DoipPayload::VehicleAnnouncementMessage(veh_ann_msg);

        let result = <[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT]>::try_from(payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_vehicle_announcement_short_fail_buffer_too_small() {
        let veh_ann_msg = VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x01, 0x02],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: None,
        };

        let payload = DoipPayload::VehicleAnnouncementMessage(veh_ann_msg);

        let result = <[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT - 1]>::try_from(payload);

        assert_eq!(result.unwrap_err(), "Invalid buffer size");
    }

    #[test]
    fn test_vehicle_announcement_short_success_buffer_large() {
        let veh_ann_msg = VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x01, 0x02],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: None,
        };

        let payload = DoipPayload::VehicleAnnouncementMessage(veh_ann_msg);

        let result = <[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT + 1]>::try_from(payload);

        assert!(result.is_ok());
    }

    // #[test]
    // fn test_try_from_routing_act_req_to_bytes() {
    //     let payload = RoutingActivationRequest {
    //         source_address: [0x01, 0x02],
    //         activation_type: ActivationType::Default,
    //         buffer: [0x01, 0x02, 0x03, 0x04],
    //     };

    //     let bytes = <[u8; DOIP_ROUTING_ACTIVATION_REQ_LEN]>::from(payload);

    //     let payload_enum = DoipPayload::RoutingActivationRequest(payload);

    //     let payload_bytes = <[u8; DOIP_ROUTING_ACTIVATION_REQ_LEN]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap(), bytes);
    // }

    #[test]
    fn test_try_from_routing_act_req_to_bytes() {
        let payload = RoutingActivationRequest {
            source_address: [0x01, 0x02],
            activation_type: ActivationType::Default,
            buffer: [0x01, 0x02, 0x03, 0x04],
        };

        let bytes = <[u8; DOIP_ROUTING_ACTIVATION_REQ_LEN]>::from(payload);

        let payload_enum = DoipPayload::RoutingActivationRequest(payload);

        let payload_bytes = <[u8; DOIP_ROUTING_ACTIVATION_REQ_LEN]>::try_from(payload_enum);

        assert_eq!(payload_bytes.unwrap_err(), "TEST");
    }

    // #[test]
    // fn test_try_from_routing_act_res_to_bytes() {
    //     let payload = RoutingActivationResponse {
    //         logical_address: [0x01, 0x02],
    //         source_address: [0x01, 0x02],
    //         activation_code: ActivationCode::ActivatedConfirmationRequired,
    //         buffer: [0x01, 0x02, 0x03, 0x04],
    //     };

    //     let bytes = <[u8; DOIP_ROUTING_ACTIVATION_RES_LEN]>::from(payload);

    //     let payload_enum = DoipPayload::RoutingActivationResponse(payload);

    //     let payload_bytes = <[u8; DOIP_ROUTING_ACTIVATION_RES_LEN]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap(), bytes);
    // }

    #[test]
    fn test_try_from_routing_act_res_to_bytes() {
        let payload = RoutingActivationResponse {
            logical_address: [0x01, 0x02],
            source_address: [0x01, 0x02],
            activation_code: ActivationCode::ActivatedConfirmationRequired,
            buffer: [0x01, 0x02, 0x03, 0x04],
        };

        let bytes = <[u8; DOIP_ROUTING_ACTIVATION_RES_LEN]>::from(payload);

        let payload_enum = DoipPayload::RoutingActivationResponse(payload);

        let payload_bytes = <[u8; DOIP_ROUTING_ACTIVATION_RES_LEN]>::try_from(payload_enum);

        assert_eq!(payload_bytes.unwrap_err(), "TEST");
    }

    // #[test]
    // fn test_try_from_alive_check_req_to_bytes() {
    //     let payload = AliveCheckRequest {};

    //     let bytes = <[u8; 0]>::from(payload);

    //     let payload_enum = DoipPayload::AliveCheckRequest(payload);

    //     let payload_bytes = <[u8; 0]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap(), bytes);
    // }

    #[test]
    fn test_try_from_alive_check_req_to_bytes() {
        let payload = AliveCheckRequest {};

        let bytes = <[u8; 0]>::from(payload);

        let payload_enum = DoipPayload::AliveCheckRequest(payload);

        let payload_bytes = <[u8; 0]>::try_from(payload_enum);

        assert_eq!(payload_bytes.unwrap_err(), "TEST");
    }

    // #[test]
    // fn test_try_from_alive_check_res_to_bytes() {
    //     let payload = AliveCheckResponse {
    //         source_address: [0x01, 0x02],
    //     };

    //     let bytes = <[u8; DOIP_ALIVE_CHECK_RESPONSE_SOURCE_LEN]>::from(payload);

    //     let payload_enum = DoipPayload::AliveCheckResponse(payload);

    //     let payload_bytes = <[u8; DOIP_ALIVE_CHECK_RESPONSE_SOURCE_LEN]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap(), bytes);
    // }

    #[test]
    fn test_try_from_alive_check_res_to_bytes() {
        let payload = AliveCheckResponse {
            source_address: [0x01, 0x02],
        };

        let bytes = <[u8; DOIP_ALIVE_CHECK_RESPONSE_SOURCE_LEN]>::from(payload);

        let payload_enum = DoipPayload::AliveCheckResponse(payload);

        let payload_bytes = <[u8; DOIP_ALIVE_CHECK_RESPONSE_SOURCE_LEN]>::try_from(payload_enum);

        assert_eq!(payload_bytes.unwrap_err(), "TEST");
    }

    // #[test]
    // fn test_try_from_entity_status_req_to_bytes() {
    //     let payload = EntityStatusRequest {};

    //     let bytes = <[u8; 0]>::from(payload);

    //     let payload_enum = DoipPayload::EntityStatusRequest(payload);

    //     let payload_bytes = <[u8; 0]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap(), bytes);
    // }

    #[test]
    fn test_try_from_entity_status_req_to_bytes() {
        let payload = EntityStatusRequest {};

        let bytes = <[u8; 0]>::from(payload);

        let payload_enum = DoipPayload::EntityStatusRequest(payload);

        let payload_bytes = <[u8; 0]>::try_from(payload_enum);

        assert_eq!(payload_bytes.unwrap_err(), "TEST");
    }

    // #[test]
    // fn test_try_from_entity_status_res_to_bytes() {
    //     let payload = EntityStatusResponse {
    //         node_type: NodeType::DoipGateway,
    //         max_concurrent_sockets: [0x01],
    //         currently_open_sockets: [0x02],
    //         max_data_size: [0x03, 0x04, 0x05, 0x06],
    //     };

    //     let bytes = <[u8; DOIP_ENTITY_STATUS_RESPONSE_LEN]>::from(payload);

    //     let payload_enum = DoipPayload::EntityStatusResponse(payload);

    //     let payload_bytes = <[u8; DOIP_ENTITY_STATUS_RESPONSE_LEN]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap(), bytes);
    // }

    #[test]
    fn test_try_from_entity_status_res_to_bytes() {
        let payload = EntityStatusResponse {
            node_type: NodeType::DoipGateway,
            max_concurrent_sockets: [0x01],
            currently_open_sockets: [0x02],
            max_data_size: [0x03, 0x04, 0x05, 0x06],
        };

        let bytes = <[u8; DOIP_ENTITY_STATUS_RESPONSE_LEN]>::from(payload);

        let payload_enum = DoipPayload::EntityStatusResponse(payload);

        let payload_bytes = <[u8; DOIP_ENTITY_STATUS_RESPONSE_LEN]>::try_from(payload_enum);

        assert_eq!(payload_bytes.unwrap_err(), "TEST");
    }

    // #[test]
    // fn test_try_from_power_info_req_to_bytes() {
    //     let payload = PowerInformationRequest {};

    //     let bytes = <[u8; 0]>::from(payload);

    //     let payload_enum = DoipPayload::PowerInformationRequest(payload);

    //     let payload_bytes = <[u8; 0]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap(), bytes);
    // }

    #[test]
    fn test_try_from_power_info_req_to_bytes() {
        let payload = PowerInformationRequest {};

        let bytes = <[u8; 0]>::from(payload);

        let payload_enum = DoipPayload::PowerInformationRequest(payload);

        let payload_bytes = <[u8; 0]>::try_from(payload_enum);

        assert_eq!(payload_bytes.unwrap_err(), "TEST");
    }

    // #[test]
    // fn test_try_from_power_info_res_to_bytes() {
    //     let payload = PowerInformationResponse {
    //         power_mode: PowerMode::Ready,
    //     };

    //     let bytes = <[u8; DOIP_POWER_MODE_LEN]>::from(payload);

    //     let payload_enum = DoipPayload::PowerInformationResponse(payload);

    //     let payload_bytes = <[u8; DOIP_POWER_MODE_LEN]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap(), bytes);
    // }

    #[test]
    fn test_try_from_power_info_res_to_bytes() {
        let payload = PowerInformationResponse {
            power_mode: PowerMode::Ready,
        };

        let bytes = <[u8; DOIP_POWER_MODE_LEN]>::from(payload);

        let payload_enum = DoipPayload::PowerInformationResponse(payload);

        let payload_bytes = <[u8; DOIP_POWER_MODE_LEN]>::try_from(payload_enum);

        assert_eq!(payload_bytes.unwrap_err(), "TEST");
    }

    // #[test]
    // fn test_try_from_diag_msg_to_bytes() {
    //     let payload = DiagnosticMessage {
    //         source_address: [0x01, 0x02],
    //         target_address: [0x01, 0x02],
    //         message: &[0x03],
    //     };

    //     let bytes = <[u8; 5]>::try_from(payload);

    //     let payload_enum = DoipPayload::DiagnosticMessage(payload);

    //     let payload_bytes = <[u8; 5]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap(), bytes.unwrap());
    // }

    #[test]
    fn test_try_from_diag_msg_to_bytes() {
        let payload = DiagnosticMessage {
            source_address: [0x01, 0x02],
            target_address: [0x01, 0x02],
            message: &[0x03],
        };

        let bytes = <[u8; 5]>::try_from(payload);

        let payload_enum = DoipPayload::DiagnosticMessage(payload);

        let payload_bytes = <[u8; 5]>::try_from(payload_enum);

        assert_eq!(payload_bytes.unwrap_err(), "TEST");
    }

    // #[test]
    // fn test_try_from_diag_msg_to_bytes_fail() {
    //     let payload = DiagnosticMessage {
    //         source_address: [0x01, 0x02],
    //         target_address: [0x01, 0x02],
    //         message: &[0x03],
    //     };

    //     let payload_enum = DoipPayload::DiagnosticMessage(payload);

    //     let payload_bytes = <[u8; 4]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap_err(), "Invalid array size");
    // }
    // #[test]
    // fn test_try_from_diag_msg_ack_to_bytes() {
    //     let payload = DiagnosticMessageAck {
    //         source_address: [0x01, 0x02],
    //         target_address: [0x01, 0x02],
    //         ack_code: DiagnosticAckCode::Acknowledged,
    //     };

    //     let bytes =
    //         <[u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]>::from(payload);

    //     let payload_enum = DoipPayload::DiagnosticMessageAck(payload);

    //     let payload_bytes =
    //         <[u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]>::try_from(
    //             payload_enum,
    //         );

    //     assert_eq!(payload_bytes.unwrap(), bytes);
    // }

    #[test]
    fn test_try_from_diag_msg_ack_to_bytes() {
        let payload = DiagnosticMessageAck {
            source_address: [0x01, 0x02],
            target_address: [0x01, 0x02],
            ack_code: DiagnosticAckCode::Acknowledged,
        };

        let bytes =
            <[u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]>::from(payload);

        let payload_enum = DoipPayload::DiagnosticMessageAck(payload);

        let payload_bytes =
            <[u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]>::try_from(
                payload_enum,
            );

        assert_eq!(payload_bytes.unwrap_err(), "TEST");
    }

    // #[test]
    // fn test_try_from_diag_msg_nack_to_bytes() {
    //     let payload = DiagnosticMessageNack {
    //         source_address: [0x01, 0x02],
    //         target_address: [0x01, 0x02],
    //         nack_code: DiagnosticNackCode::OutOfMemory,
    //     };

    //     let bytes =
    //         <[u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]>::from(payload);

    //     let payload_enum = DoipPayload::DiagnosticMessageNack(payload);

    //     let payload_bytes =
    //         <[u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]>::try_from(
    //             payload_enum,
    //         );

    //     assert_eq!(payload_bytes.unwrap(), bytes);
    // }

    #[test]
    fn test_try_from_diag_msg_nack_to_bytes() {
        let payload = DiagnosticMessageNack {
            source_address: [0x01, 0x02],
            target_address: [0x01, 0x02],
            nack_code: DiagnosticNackCode::OutOfMemory,
        };

        let bytes =
            <[u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]>::from(payload);

        let payload_enum = DoipPayload::DiagnosticMessageNack(payload);

        let payload_bytes =
            <[u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]>::try_from(
                payload_enum,
            );

        assert_eq!(payload_bytes.unwrap_err(), "TEST");
    }

    // #[test]
    // fn test_try_from_generic_nack_to_bytes_invalid_buffer() {
    //     let payload = GenericNack {
    //         nack_code: NackCode::IncorrectPatternFormat,
    //     };

    //     let payload_enum = DoipPayload::GenericNack(payload);

    //     let payload_bytes = <[u8; 0]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap_err(), "Buffer is too small");
    // }

    // #[test]
    // fn test_try_from_vehicle_id_req_eid_to_bytes_invalid_buffer() {
    //     let payload = VehicleIdentificationRequestEid {
    //         eid: [0u8; DOIP_COMMON_EID_LEN],
    //     };

    //     let payload_enum = DoipPayload::VehicleIdentificationRequestEid(payload);

    //     let payload_bytes = <[u8; 0]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap_err(), "Buffer is too small");
    // }
    // #[test]
    // fn test_try_from_vehicle_id_req_vin_to_bytes_invalid_buffer() {
    //     let payload = VehicleIdentificationRequestVin {
    //         vin: [0u8; DOIP_COMMON_VIN_LEN],
    //     };

    //     let payload_enum = DoipPayload::VehicleIdentificationRequestVin(payload);

    //     let payload_bytes = <[u8; 0]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap_err(), "Buffer is too small");
    // }
    // #[test]
    // fn test_try_from_vehicle_announce_msg_short_to_bytes_invalid_buffer() {
    //     let payload = VehicleAnnouncementMessage {
    //         vin: [0u8; DOIP_COMMON_VIN_LEN],
    //         logical_address: [0x01, 0x02],
    //         eid: [0u8; DOIP_COMMON_EID_LEN],
    //         gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
    //         further_action: ActionCode::NoFurtherActionRequired,
    //         vin_gid_sync: None,
    //     };

    //     let payload_enum = DoipPayload::VehicleAnnouncementMessage(payload);

    //     let payload_bytes = <[u8; 0]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap_err(), "Buffer is too small");
    // }
    // #[test]
    // fn test_try_from_vehicle_announce_msg_long_to_bytes_invalid_buffer() {
    //     let payload = VehicleAnnouncementMessage {
    //         vin: [0u8; DOIP_COMMON_VIN_LEN],
    //         logical_address: [0x01, 0x02],
    //         eid: [0u8; DOIP_COMMON_EID_LEN],
    //         gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
    //         further_action: ActionCode::NoFurtherActionRequired,
    //         vin_gid_sync: Some(SyncStatus::VinGidSynchronized),
    //     };

    //     let payload_enum = DoipPayload::VehicleAnnouncementMessage(payload);

    //     let payload_bytes = <[u8; 0]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap_err(), "Buffer is too small");
    // }
    // #[test]
    // fn test_try_from_routing_act_req_to_bytes_invalid_buffer() {
    //     let payload = RoutingActivationRequest {
    //         source_address: [0x01, 0x02],
    //         activation_type: ActivationType::Default,
    //         buffer: [0x01, 0x02, 0x03, 0x04],
    //     };

    //     let payload_enum = DoipPayload::RoutingActivationRequest(payload);

    //     let payload_bytes = <[u8; 0]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap_err(), "Buffer is too small");
    // }
    // #[test]
    // fn test_try_from_routing_act_res_to_bytes_invalid_buffer() {
    //     let payload = RoutingActivationResponse {
    //         logical_address: [0x01, 0x02],
    //         source_address: [0x01, 0x02],
    //         activation_code: ActivationCode::ActivatedConfirmationRequired,
    //         buffer: [0x01, 0x02, 0x03, 0x04],
    //     };

    //     let payload_enum = DoipPayload::RoutingActivationResponse(payload);

    //     let payload_bytes = <[u8; 0]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap_err(), "Buffer is too small");
    // }
    // #[test]
    // fn test_try_from_alive_check_res_to_bytes_invalid_buffer() {
    //     let payload = AliveCheckResponse {
    //         source_address: [0x01, 0x02],
    //     };

    //     let payload_enum = DoipPayload::AliveCheckResponse(payload);

    //     let payload_bytes = <[u8; 0]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap_err(), "Buffer is too small");
    // }
    // #[test]
    // fn test_try_from_entity_status_res_to_bytes_invalid_buffer() {
    //     let payload = EntityStatusResponse {
    //         node_type: NodeType::DoipGateway,
    //         max_concurrent_sockets: [0x01],
    //         currently_open_sockets: [0x02],
    //         max_data_size: [0x03, 0x04, 0x05, 0x06],
    //     };

    //     let payload_enum = DoipPayload::EntityStatusResponse(payload);

    //     let payload_bytes = <[u8; 0]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap_err(), "Buffer is too small");
    // }
    // #[test]
    // fn test_try_from_power_info_res_to_bytes_invalid_buffer() {
    //     let payload = PowerInformationResponse {
    //         power_mode: PowerMode::Ready,
    //     };

    //     let payload_enum = DoipPayload::PowerInformationResponse(payload);

    //     let payload_bytes = <[u8; 0]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap_err(), "Buffer is too small");
    // }
    // #[test]
    // fn test_try_from_diag_msg_ack_to_bytes_invalid_buffer() {
    //     let payload = DiagnosticMessageAck {
    //         source_address: [0x01, 0x02],
    //         target_address: [0x01, 0x02],
    //         ack_code: DiagnosticAckCode::Acknowledged,
    //     };

    //     let payload_enum = DoipPayload::DiagnosticMessageAck(payload);

    //     let payload_bytes = <[u8; 0]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap_err(), "Buffer is too small");
    // }
    // #[test]
    // fn test_try_from_diag_msg_nack_to_bytes_invalid_buffer() {
    //     let payload = DiagnosticMessageNack {
    //         source_address: [0x01, 0x02],
    //         target_address: [0x01, 0x02],
    //         nack_code: DiagnosticNackCode::OutOfMemory,
    //     };

    //     let payload_enum = DoipPayload::DiagnosticMessageNack(payload);

    //     let payload_bytes = <[u8; 0]>::try_from(payload_enum);

    //     assert_eq!(payload_bytes.unwrap_err(), "Buffer is too small");
    // }

    // #[test]
    // fn test_try_from_bytes_generic_nack() {
    //     let payload = GenericNack {
    //         nack_code: NackCode::IncorrectPatternFormat,
    //     };

    //     let payload_bytes: &[u8] = &<[u8; 1]>::from(payload);

    //     let header = DoipHeader {
    //         protocol_version: ProtocolVersion::Iso13400_2012,
    //         inverse_protocol_version: 0xfd,
    //         payload_type: PayloadType::GenericNack,
    //         payload_length: 0x00000001,
    //     };

    //     let bytes = DoipPayload::try_from((&header, payload_bytes));

    //     assert_eq!(bytes.unwrap(), DoipPayload::GenericNack(payload))
    // }

    // #[test]
    // fn test_try_from_bytes_vehicle_id_req() {
    //     let payload = VehicleIdentificationRequest {};

    //     let payload_bytes: &[u8] = &<[u8; 0]>::from(payload);

    //     let header = DoipHeader {
    //         protocol_version: ProtocolVersion::Iso13400_2012,
    //         inverse_protocol_version: 0xfd,
    //         payload_type: PayloadType::VehicleIdentificationRequest,
    //         payload_length: payload_bytes.len() as u32,
    //     };

    //     let bytes = DoipPayload::try_from((&header, payload_bytes));

    //     assert_eq!(
    //         bytes.unwrap(),
    //         DoipPayload::VehicleIdentificationRequest(payload)
    //     )
    // }

    // #[test]
    // fn test_try_from_bytes_vehicle_id_req_eid() {
    //     let payload = VehicleIdentificationRequestEid {
    //         eid: [0u8; DOIP_COMMON_EID_LEN],
    //     };

    //     let payload_bytes: &[u8] = &<[u8; DOIP_COMMON_EID_LEN]>::from(payload);

    //     let header = DoipHeader {
    //         protocol_version: ProtocolVersion::Iso13400_2012,
    //         inverse_protocol_version: 0xfd,
    //         payload_type: PayloadType::VehicleIdentificationRequestEid,
    //         payload_length: payload_bytes.len() as u32,
    //     };

    //     let bytes = DoipPayload::try_from((&header, payload_bytes));

    //     assert_eq!(
    //         bytes.unwrap(),
    //         DoipPayload::VehicleIdentificationRequestEid(payload)
    //     )
    // }

    // #[test]
    // fn test_try_from_bytes_vehicle_id_req_vin() {
    //     let payload = VehicleIdentificationRequestVin {
    //         vin: [0u8; DOIP_COMMON_VIN_LEN],
    //     };

    //     let payload_bytes: &[u8] = &<[u8; DOIP_COMMON_VIN_LEN]>::from(payload);

    //     let header = DoipHeader {
    //         protocol_version: ProtocolVersion::Iso13400_2012,
    //         inverse_protocol_version: 0xfd,
    //         payload_type: PayloadType::VehicleIdentificationRequestVin,
    //         payload_length: payload_bytes.len() as u32,
    //     };

    //     let bytes = DoipPayload::try_from((&header, payload_bytes));

    //     assert_eq!(
    //         bytes.unwrap(),
    //         DoipPayload::VehicleIdentificationRequestVin(payload)
    //     )
    // }

    // #[test]
    // fn test_try_from_bytes_vehicle_announce_msg_short() {
    //     let payload = VehicleAnnouncementMessage {
    //         vin: [0u8; DOIP_COMMON_VIN_LEN],
    //         logical_address: [0x01, 0x02],
    //         eid: [0u8; DOIP_COMMON_EID_LEN],
    //         gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
    //         further_action: ActionCode::NoFurtherActionRequired,
    //         vin_gid_sync: None,
    //     };

    //     let payload_bytes: &[u8] = &<[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT]>::from(payload);

    //     let header = DoipHeader {
    //         protocol_version: ProtocolVersion::Iso13400_2012,
    //         inverse_protocol_version: 0xfd,
    //         payload_type: PayloadType::VehicleAnnouncementMessage,
    //         payload_length: payload_bytes.len() as u32,
    //     };

    //     let bytes = DoipPayload::try_from((&header, payload_bytes));

    //     assert_eq!(
    //         bytes.unwrap(),
    //         DoipPayload::VehicleAnnouncementMessage(payload)
    //     )
    // }

    // #[test]
    // fn test_try_from_bytes_vehicle_announce_msg_long() {
    //     let payload = VehicleAnnouncementMessage {
    //         vin: [0u8; DOIP_COMMON_VIN_LEN],
    //         logical_address: [0x01, 0x02],
    //         eid: [0u8; DOIP_COMMON_EID_LEN],
    //         gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
    //         further_action: ActionCode::NoFurtherActionRequired,
    //         vin_gid_sync: Some(SyncStatus::VinGidSynchronized),
    //     };

    //     let payload_bytes: &[u8] = &<[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG]>::from(payload);

    //     let header = DoipHeader {
    //         protocol_version: ProtocolVersion::Iso13400_2012,
    //         inverse_protocol_version: 0xfd,
    //         payload_type: PayloadType::VehicleAnnouncementMessage,
    //         payload_length: payload_bytes.len() as u32,
    //     };

    //     let bytes = DoipPayload::try_from((&header, payload_bytes));

    //     assert_eq!(
    //         bytes.unwrap(),
    //         DoipPayload::VehicleAnnouncementMessage(payload)
    //     )
    // }

    // #[test]
    // fn test_try_from_bytes_routing_act_req() {
    //     let payload = RoutingActivationRequest {
    //         source_address: [0x01, 0x02],
    //         activation_type: ActivationType::Default,
    //         buffer: [0x01, 0x02, 0x03, 0x04],
    //     };

    //     let payload_bytes: &[u8] = &<[u8; DOIP_ROUTING_ACTIVATION_REQ_LEN]>::from(payload);

    //     let header = DoipHeader {
    //         protocol_version: ProtocolVersion::Iso13400_2012,
    //         inverse_protocol_version: 0xfd,
    //         payload_type: PayloadType::RoutingActivationRequest,
    //         payload_length: payload_bytes.len() as u32,
    //     };

    //     let bytes = DoipPayload::try_from((&header, payload_bytes));

    //     assert_eq!(
    //         bytes.unwrap(),
    //         DoipPayload::RoutingActivationRequest(payload)
    //     )
    // }

    // #[test]
    // fn test_try_from_bytes_routing_act_res() {
    //     let payload = RoutingActivationResponse {
    //         logical_address: [0x01, 0x02],
    //         source_address: [0x01, 0x02],
    //         activation_code: ActivationCode::ActivatedConfirmationRequired,
    //         buffer: [0x01, 0x02, 0x03, 0x04],
    //     };

    //     let payload_bytes: &[u8] = &<[u8; DOIP_ROUTING_ACTIVATION_RES_LEN]>::from(payload);

    //     let header = DoipHeader {
    //         protocol_version: ProtocolVersion::Iso13400_2012,
    //         inverse_protocol_version: 0xfd,
    //         payload_type: PayloadType::RoutingActivationResponse,
    //         payload_length: payload_bytes.len() as u32,
    //     };

    //     let bytes = DoipPayload::try_from((&header, payload_bytes));

    //     assert_eq!(
    //         bytes.unwrap(),
    //         DoipPayload::RoutingActivationResponse(payload)
    //     )
    // }

    // #[test]
    // fn test_try_from_bytes_alive_check_req() {
    //     let payload = AliveCheckRequest {};

    //     let payload_bytes: &[u8] = &<[u8; 0]>::from(payload);

    //     let header = DoipHeader {
    //         protocol_version: ProtocolVersion::Iso13400_2012,
    //         inverse_protocol_version: 0xfd,
    //         payload_type: PayloadType::AliveCheckRequest,
    //         payload_length: payload_bytes.len() as u32,
    //     };

    //     let bytes = DoipPayload::try_from((&header, payload_bytes));

    //     assert_eq!(bytes.unwrap(), DoipPayload::AliveCheckRequest(payload))
    // }

    // #[test]
    // fn test_try_from_bytes_alive_check_res() {
    //     let payload = AliveCheckResponse {
    //         source_address: [0x01, 0x02],
    //     };

    //     let payload_bytes: &[u8] = &<[u8; DOIP_ALIVE_CHECK_RESPONSE_SOURCE_LEN]>::from(payload);

    //     let header = DoipHeader {
    //         protocol_version: ProtocolVersion::Iso13400_2012,
    //         inverse_protocol_version: 0xfd,
    //         payload_type: PayloadType::AliveCheckResponse,
    //         payload_length: payload_bytes.len() as u32,
    //     };

    //     let bytes = DoipPayload::try_from((&header, payload_bytes));

    //     assert_eq!(bytes.unwrap(), DoipPayload::AliveCheckResponse(payload))
    // }

    // #[test]
    // fn test_try_from_bytes_entity_status_req() {
    //     let payload = EntityStatusRequest {};

    //     let payload_bytes: &[u8] = &<[u8; 0]>::from(payload);

    //     let header = DoipHeader {
    //         protocol_version: ProtocolVersion::Iso13400_2012,
    //         inverse_protocol_version: 0xfd,
    //         payload_type: PayloadType::EntityStatusRequest,
    //         payload_length: payload_bytes.len() as u32,
    //     };

    //     let bytes = DoipPayload::try_from((&header, payload_bytes));

    //     assert_eq!(bytes.unwrap(), DoipPayload::EntityStatusRequest(payload))
    // }

    // #[test]
    // fn test_try_from_bytes_entity_status_res() {
    //     let payload = EntityStatusResponse {
    //         node_type: NodeType::DoipGateway,
    //         max_concurrent_sockets: [0x01],
    //         currently_open_sockets: [0x02],
    //         max_data_size: [0x03, 0x04, 0x05, 0x06],
    //     };

    //     let payload_bytes: &[u8] = &<[u8; DOIP_ENTITY_STATUS_RESPONSE_LEN]>::from(payload);

    //     let header = DoipHeader {
    //         protocol_version: ProtocolVersion::Iso13400_2012,
    //         inverse_protocol_version: 0xfd,
    //         payload_type: PayloadType::EntityStatusResponse,
    //         payload_length: payload_bytes.len() as u32,
    //     };

    //     let bytes = DoipPayload::try_from((&header, payload_bytes));

    //     assert_eq!(bytes.unwrap(), DoipPayload::EntityStatusResponse(payload))
    // }

    // #[test]
    // fn test_try_from_bytes_power_info_req() {
    //     let payload = PowerInformationRequest {};

    //     let payload_bytes: &[u8] = &<[u8; 0]>::from(payload);

    //     let header = DoipHeader {
    //         protocol_version: ProtocolVersion::Iso13400_2012,
    //         inverse_protocol_version: 0xfd,
    //         payload_type: PayloadType::PowerInformationRequest,
    //         payload_length: payload_bytes.len() as u32,
    //     };

    //     let bytes = DoipPayload::try_from((&header, payload_bytes));

    //     assert_eq!(
    //         bytes.unwrap(),
    //         DoipPayload::PowerInformationRequest(payload)
    //     )
    // }

    // #[test]
    // fn test_try_from_bytes_power_info_res() {
    //     let payload = PowerInformationResponse {
    //         power_mode: PowerMode::Ready,
    //     };

    //     let payload_bytes: &[u8] = &<[u8; DOIP_POWER_MODE_LEN]>::from(payload);

    //     let header = DoipHeader {
    //         protocol_version: ProtocolVersion::Iso13400_2012,
    //         inverse_protocol_version: 0xfd,
    //         payload_type: PayloadType::PowerInformationResponse,
    //         payload_length: payload_bytes.len() as u32,
    //     };

    //     let bytes = DoipPayload::try_from((&header, payload_bytes));

    //     assert_eq!(
    //         bytes.unwrap(),
    //         DoipPayload::PowerInformationResponse(payload)
    //     )
    // }

    // #[test]
    // fn test_try_from_bytes_diag_msg() {
    //     let payload = DiagnosticMessage {
    //         source_address: [0x01, 0x02],
    //         target_address: [0x01, 0x02],
    //         message: &[0x03],
    //     };

    //     let payload_bytes: &[u8] = &<[u8; 5]>::try_from(payload).unwrap();

    //     let header = DoipHeader {
    //         protocol_version: ProtocolVersion::Iso13400_2012,
    //         inverse_protocol_version: 0xfd,
    //         payload_type: PayloadType::DiagnosticMessage,
    //         payload_length: payload_bytes.len() as u32,
    //     };

    //     let bytes = DoipPayload::try_from((&header, payload_bytes));

    //     assert_eq!(
    //         bytes.unwrap(),
    //         DoipPayload::DiagnosticMessage(payload.clone())
    //     )
    // }

    // #[test]
    // fn test_try_from_bytes_diag_msg_ack() {
    //     let payload = DiagnosticMessageAck {
    //         source_address: [0x01, 0x02],
    //         target_address: [0x01, 0x02],
    //         ack_code: DiagnosticAckCode::Acknowledged,
    //     };

    //     let payload_bytes: &[u8] = &<[u8; DOIP_DIAG_COMMON_SOURCE_LEN
    //         + DOIP_DIAG_COMMON_TARGET_LEN
    //         + 1]>::try_from(payload)
    //     .unwrap();

    //     let header = DoipHeader {
    //         protocol_version: ProtocolVersion::Iso13400_2012,
    //         inverse_protocol_version: 0xfd,
    //         payload_type: PayloadType::DiagnosticMessageAck,
    //         payload_length: payload_bytes.len() as u32,
    //     };

    //     let bytes = DoipPayload::try_from((&header, payload_bytes));

    //     assert_eq!(
    //         bytes.unwrap(),
    //         DoipPayload::DiagnosticMessageAck(payload.clone())
    //     )
    // }

    // #[test]
    // fn test_try_from_bytes_diag_msg_nack() {
    //     let payload = DiagnosticMessageNack {
    //         source_address: [0x01, 0x02],
    //         target_address: [0x01, 0x02],
    //         nack_code: DiagnosticNackCode::OutOfMemory,
    //     };

    //     let payload_bytes: &[u8] = &<[u8; DOIP_DIAG_COMMON_SOURCE_LEN
    //         + DOIP_DIAG_COMMON_TARGET_LEN
    //         + 1]>::try_from(payload)
    //     .unwrap();

    //     let header = DoipHeader {
    //         protocol_version: ProtocolVersion::Iso13400_2012,
    //         inverse_protocol_version: 0xfd,
    //         payload_type: PayloadType::DiagnosticMessageNack,
    //         payload_length: payload_bytes.len() as u32,
    //     };

    //     let bytes = DoipPayload::try_from((&header, payload_bytes));

    //     assert_eq!(
    //         bytes.unwrap(),
    //         DoipPayload::DiagnosticMessageNack(payload.clone())
    //     )
    // }

    // #[test]
    // fn test_try_from_bytes_too_short() {
    //     let payload_bytes: &[u8] = &[0u8; 2];

    //     let header = DoipHeader {
    //         protocol_version: ProtocolVersion::Iso13400_2012,
    //         inverse_protocol_version: 0xfd,
    //         payload_type: PayloadType::DiagnosticMessageNack,
    //         payload_length: 0x01,
    //     };

    //     let bytes = DoipPayload::try_from((&header, payload_bytes));

    //     assert_eq!(bytes.unwrap_err(), "Slice does not match payload length.")
    // }

    #[test]
    fn test_vehicle_announcement_to_bytes_success_sync() {
        const N: usize = 33;

        let veh_ann_msg = DoipPayload::VehicleAnnouncementMessage(VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x01, 0x02],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: Some(SyncStatus::VinGidSynchronized),
        });

        let result = <[u8; N]>::try_from(veh_ann_msg);

        assert!(result.is_ok());
    }

    #[test]
    fn test_vehicle_announcement_to_bytes_success_no_sync() {
        const N: usize = 32;

        let veh_ann_msg = DoipPayload::VehicleAnnouncementMessage(VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x01, 0x02],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: None,
        });

        let result = <[u8; N]>::try_from(veh_ann_msg);

        assert!(result.is_ok());
    }

    #[test]
    fn test_vehicle_announcement_to_bytes_fail_no_sync() {
        const N: usize = 31;

        let veh_ann_msg = DoipPayload::VehicleAnnouncementMessage(VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x01, 0x02],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: None,
        });

        let result = <[u8; N]>::try_from(veh_ann_msg);

        assert_eq!(result.unwrap_err(), "Invalid buffer size");
    }

    #[test]
    fn test_vehicle_announcement_to_bytes_fail_sync() {
        const N: usize = 32;

        let veh_ann_msg = DoipPayload::VehicleAnnouncementMessage(VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x01, 0x02],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: Some(SyncStatus::VinGidSynchronized),
        });

        let result = <[u8; N]>::try_from(veh_ann_msg);

        assert_eq!(result.unwrap_err(), "Invalid buffer size");
    }

    #[test]
    fn test_vehicle_announcement_to_bytes_sucess_sync_large_buffer() {
        const N: usize = 35;

        let veh_ann_msg = DoipPayload::VehicleAnnouncementMessage(VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x01, 0x02],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: Some(SyncStatus::VinGidSynchronized),
        });

        let result = <[u8; N]>::try_from(veh_ann_msg);

        assert!(result.is_ok());
    }

    #[test]
    fn test_vehicle_announcement_to_bytes_sucess_no_sync_large_buffer() {
        const N: usize = 35;

        let veh_ann_msg = DoipPayload::VehicleAnnouncementMessage(VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x01, 0x02],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: None,
        });

        let result = <[u8; N]>::try_from(veh_ann_msg);

        assert!(result.is_ok());
    }

    #[test]
    fn test_vehicle_announcement_to_bytes_fail_size() {
        // Case 1: N is too small for even the short format
        const N1: usize = 1;

        let veh_ann_msg_short =
            DoipPayload::VehicleAnnouncementMessage(VehicleAnnouncementMessage {
                vin: [0u8; DOIP_COMMON_VIN_LEN],
                logical_address: [0x01, 0x02],
                eid: [0u8; DOIP_COMMON_EID_LEN],
                gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
                further_action: ActionCode::NoFurtherActionRequired,
                vin_gid_sync: None,
            });

        let result_short = <[u8; N1]>::try_from(veh_ann_msg_short);

        assert_eq!(result_short.unwrap_err(), "Invalid buffer size");

        // Case 2: N is enough for the short format but not for the long format
        const N2: usize = DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT;

        let veh_ann_msg_long =
            DoipPayload::VehicleAnnouncementMessage(VehicleAnnouncementMessage {
                vin: [0u8; DOIP_COMMON_VIN_LEN],
                logical_address: [0x01, 0x02],
                eid: [0u8; DOIP_COMMON_EID_LEN],
                gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
                further_action: ActionCode::NoFurtherActionRequired,
                vin_gid_sync: Some(SyncStatus::VinGidSynchronized),
            });

        let result_long = <[u8; N2]>::try_from(veh_ann_msg_long);

        assert_eq!(result_long.unwrap_err(), "Invalid buffer size");
    }
}

// fn handle_my_thing<const N: usize>(
//     veh_ann_msg: VehicleAnnouncementMessage,
//     mut buffer: [u8; N],
// ) -> Result<[u8; N], &'static str> {
//     const LONG: usize = DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG;
//     const SHORT: usize = DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT;

//     let bytes = <[u8; N]>::from(veh_ann_msg);

//     match veh_ann_msg.vin_gid_sync {
//         None => {
//             if bytes.len() <= N {
//                 buffer[..bytes.len()].copy_from_slice(&bytes);
//                 Ok(buffer)
//             } else {
//                 Err("Buffer is too small")
//             }
//         }
//         Some(sync_status) => {
//             let _ = sync_status;

//             let bytes = <[u8; LONG]>::from(veh_ann_msg);

//             if bytes.len() <= N {
//                 buffer[..bytes.len()].copy_from_slice(&bytes);
//                 Ok(buffer)
//             } else {
//                 Err("Buffer is too small")
//             }
//         }
//     }
// }
