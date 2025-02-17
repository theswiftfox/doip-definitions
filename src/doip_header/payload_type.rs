use crate::definitions::{
    DOIP_ALIVE_CHECK_REQUEST, DOIP_ALIVE_CHECK_RESPONSE, DOIP_DIAGNOSTIC_MESSAGE,
    DOIP_DIAGNOSTIC_MESSAGE_ACK, DOIP_DIAGNOSTIC_MESSAGE_NACK, DOIP_ENTITY_STATUS_REQUEST,
    DOIP_ENTITY_STATUS_RESPONSE, DOIP_GENERIC_NACK, DOIP_POWER_INFORMATION_REQUEST,
    DOIP_POWER_INFORMATION_RESPONSE, DOIP_ROUTING_ACTIVATION_REQUEST,
    DOIP_ROUTING_ACTIVATION_RESPONSE, DOIP_TYPE_LEN, DOIP_VEHICLE_ANNOUNCEMENT_MESSAGE,
    DOIP_VEHICLE_IDENTIFICATION_REQ, DOIP_VEHICLE_IDENTIFICATION_REQ_EID,
    DOIP_VEHICLE_IDENTIFICATION_REQ_VIN,
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

impl From<PayloadType> for [u8; DOIP_TYPE_LEN] {
    fn from(payload_type: PayloadType) -> Self {
        (payload_type as u16).to_be_bytes()
    }
}

impl TryFrom<[u8; DOIP_TYPE_LEN]> for PayloadType {
    type Error = &'static str;

    fn try_from(value: [u8; DOIP_TYPE_LEN]) -> Result<Self, Self::Error> {
        let val: u16 = (u16::from(value[0]) << 8) | u16::from(value[1]);

        match val {
            DOIP_GENERIC_NACK => Ok(PayloadType::GenericNack),
            DOIP_VEHICLE_IDENTIFICATION_REQ => Ok(PayloadType::VehicleIdentificationRequest),
            DOIP_VEHICLE_IDENTIFICATION_REQ_EID => Ok(PayloadType::VehicleIdentificationRequestEid),
            DOIP_VEHICLE_IDENTIFICATION_REQ_VIN => Ok(PayloadType::VehicleIdentificationRequestVin),
            DOIP_VEHICLE_ANNOUNCEMENT_MESSAGE => Ok(PayloadType::VehicleAnnouncementMessage),
            DOIP_ROUTING_ACTIVATION_REQUEST => Ok(PayloadType::RoutingActivationRequest),
            DOIP_ROUTING_ACTIVATION_RESPONSE => Ok(PayloadType::RoutingActivationResponse),
            DOIP_ALIVE_CHECK_REQUEST => Ok(PayloadType::AliveCheckRequest),
            DOIP_ALIVE_CHECK_RESPONSE => Ok(PayloadType::AliveCheckResponse),
            DOIP_ENTITY_STATUS_REQUEST => Ok(PayloadType::EntityStatusRequest),
            DOIP_ENTITY_STATUS_RESPONSE => Ok(PayloadType::EntityStatusResponse),
            DOIP_POWER_INFORMATION_REQUEST => Ok(PayloadType::PowerInformationRequest),
            DOIP_POWER_INFORMATION_RESPONSE => Ok(PayloadType::PowerInformationResponse),
            DOIP_DIAGNOSTIC_MESSAGE => Ok(PayloadType::DiagnosticMessage),
            DOIP_DIAGNOSTIC_MESSAGE_ACK => Ok(PayloadType::DiagnosticMessageAck),
            DOIP_DIAGNOSTIC_MESSAGE_NACK => Ok(PayloadType::DiagnosticMessageNack),
            _ => Err("Invalid PayloadType."),
        }
    }
}
