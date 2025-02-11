use crate::{
    definitions::{DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN},
    header::PayloadType,
    DoipPayload,
};

/// A UDS Message to a specific target address.
///
/// `DiagnosticMessage` is the most utilised payload type due to the amount of actions
/// a diagnostic tester can do using the UDS protocol. This crate will not handle the UDS
/// protocol however, one will be developed to enhance developer tooling.
#[derive(Clone, Debug)]
pub struct DiagnosticMessage<'a> {
    /// The source address of the responding `DoIP` Entity
    pub source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN],

    /// The target address of the requesting `DoIP` Entity
    pub target_address: [u8; DOIP_DIAG_COMMON_TARGET_LEN],

    /// Message containing the UDS protocol message
    pub message: &'a [u8],
}

impl DoipPayload for DiagnosticMessage<'_> {
    fn payload_type(&self) -> PayloadType {
        PayloadType::DiagnosticMessage
    }
}
