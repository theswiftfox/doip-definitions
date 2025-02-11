use crate::{
    definitions::{DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN},
    header::PayloadType,
    payload::DiagnosticAckCode,
    DoipPayload,
};

/// Postive acknowledgement of a `DiagnosticMessage`.
///
/// Containing the source and target entity addresses, as well as the `DiagnosticAckCode`
/// for the `DiagnosticMessage` initially sent by the target entity.
#[derive(Copy, Clone, Debug)]
pub struct DiagnosticMessageAck {
    /// The source address of the responding `DoIP` Entity
    pub source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN],

    /// The target address of the requesting `DoIP` Entity
    pub target_address: [u8; DOIP_DIAG_COMMON_TARGET_LEN],

    /// The positive acknowledgement code
    pub ack_code: DiagnosticAckCode,
}

impl DoipPayload for DiagnosticMessageAck {
    fn payload_type(&self) -> PayloadType {
        PayloadType::DiagnosticMessageAck
    }
}
