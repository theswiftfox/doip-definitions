use crate::{
    definitions::{DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN},
    header::{DoipPayload, PayloadType},
    message::DiagnosticNackCode,
};

/// Negative acknowledgement of a `DiagnosticMessage`.
///
/// Containing the source and target entity addresses, as well as the `DiagnosticNackCode`
/// for the `DiagnosticMessage` initially sent by the target entity.
#[derive(Copy, Clone, Debug)]
pub struct DiagnosticMessageNack {
    /// The source address of the responding `DoIP` Entity
    pub source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN],

    /// The target address of the requesting `DoIP` Entity
    pub target_address: [u8; DOIP_DIAG_COMMON_TARGET_LEN],

    /// The negative acknowledgement code
    pub nack_code: DiagnosticNackCode,
}

impl DoipPayload for DiagnosticMessageNack {
    fn payload_type(&self) -> PayloadType {
        PayloadType::DiagnosticMessageNack
    }
}
