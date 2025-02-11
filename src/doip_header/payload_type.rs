use crate::{
    definitions::{
        DOIP_ALIVE_CHECK_REQUEST, DOIP_ALIVE_CHECK_RESPONSE, DOIP_DIAGNOSTIC_MESSAGE,
        DOIP_DIAGNOSTIC_MESSAGE_ACK, DOIP_DIAGNOSTIC_MESSAGE_NACK, DOIP_ENTITY_STATUS_REQUEST,
        DOIP_ENTITY_STATUS_RESPONSE, DOIP_GENERIC_NACK, DOIP_POWER_INFORMATION_REQUEST,
        DOIP_POWER_INFORMATION_RESPONSE, DOIP_ROUTING_ACTIVATION_REQUEST,
        DOIP_ROUTING_ACTIVATION_RESPONSE, DOIP_VEHICLE_ANNOUNCEMENT_MESSAGE,
        DOIP_VEHICLE_IDENTIFICATION_REQ, DOIP_VEHICLE_IDENTIFICATION_REQ_EID,
        DOIP_VEHICLE_IDENTIFICATION_REQ_VIN,
    },
    DoipPayload,
};

/// Defines the variants of payloads available to `DoIP`.
///
/// `PayloadType` values map to the `u16` representing the bytes it makes up
/// within the `DoIP` packet.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u16)]
pub enum PayloadType {
    /// Generic Negative Acknowledge
    GenericNack = DOIP_GENERIC_NACK,

    /// Vehicle Identification Request
    VehicleIdentificationRequest = DOIP_VEHICLE_IDENTIFICATION_REQ,

    /// Vehicle Identification Request by EID
    VehicleIdentificationRequestEid = DOIP_VEHICLE_IDENTIFICATION_REQ_EID,

    /// Vehicle Identification Request by VIN
    VehicleIdentificationRequestVin = DOIP_VEHICLE_IDENTIFICATION_REQ_VIN,

    /// Vehicle Announcement Message
    VehicleAnnouncementMessage = DOIP_VEHICLE_ANNOUNCEMENT_MESSAGE,

    /// Routing Activation Request
    RoutingActivationRequest = DOIP_ROUTING_ACTIVATION_REQUEST,

    /// Routing Activation Response
    RoutingActivationResponse = DOIP_ROUTING_ACTIVATION_RESPONSE,

    /// Alive Check Request
    AliveCheckRequest = DOIP_ALIVE_CHECK_REQUEST,

    /// Alive Check Response
    AliveCheckResponse = DOIP_ALIVE_CHECK_RESPONSE,

    /// Entity Status Request
    EntityStatusRequest = DOIP_ENTITY_STATUS_REQUEST,

    /// Entity Status Response
    EntityStatusResponse = DOIP_ENTITY_STATUS_RESPONSE,

    /// Power Information Request
    PowerInformationRequest = DOIP_POWER_INFORMATION_REQUEST,

    /// Power Information Response
    PowerInformationResponse = DOIP_POWER_INFORMATION_RESPONSE,

    /// Diagnostic Message
    DiagnosticMessage = DOIP_DIAGNOSTIC_MESSAGE,

    /// Diagnostic Message Acknowledgement
    DiagnosticMessageAck = DOIP_DIAGNOSTIC_MESSAGE_ACK,

    /// Diagnostic Message Negative Acknowledgement
    DiagnosticMessageNack = DOIP_DIAGNOSTIC_MESSAGE_NACK,
}

impl DoipPayload for PayloadType {
    fn payload_type(&self) -> PayloadType {
        *self
    }
}
