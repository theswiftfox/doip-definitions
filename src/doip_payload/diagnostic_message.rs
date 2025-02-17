use crate::definitions::{DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN};

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

impl<'a, const N: usize> From<DiagnosticMessage<'a>> for [u8; N] {
    fn from(diagnostic_message: DiagnosticMessage<'a>) -> Self {
        let message_len = diagnostic_message.message.len();

        assert!(
            N == DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + message_len,
            "Invalid array size"
        );

        let mut buffer = [0u8; N];
        let mut offset = 0;

        buffer[offset..offset + DOIP_DIAG_COMMON_SOURCE_LEN]
            .copy_from_slice(&diagnostic_message.source_address);
        offset += DOIP_DIAG_COMMON_SOURCE_LEN;

        buffer[offset..offset + DOIP_DIAG_COMMON_TARGET_LEN]
            .copy_from_slice(&diagnostic_message.target_address);
        offset += DOIP_DIAG_COMMON_TARGET_LEN;

        buffer[offset..].copy_from_slice(diagnostic_message.message);

        buffer
    }
}

impl<'a> TryFrom<&'a [u8]> for DiagnosticMessage<'a> {
    type Error = &'static str;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if value.len() < DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN {
            return Err("Input slice too short to be a valid DiagnosticMessage");
        }

        let (source_slice, rest) = value.split_at(DOIP_DIAG_COMMON_SOURCE_LEN);
        let (target_slice, message) = rest.split_at(DOIP_DIAG_COMMON_TARGET_LEN);

        let source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN] = source_slice
            .try_into()
            .map_err(|_| "Invalid source address length")?;

        let target_address: [u8; DOIP_DIAG_COMMON_TARGET_LEN] = target_slice
            .try_into()
            .map_err(|_| "Invalid target address length")?;

        Ok(DiagnosticMessage {
            source_address,
            target_address,
            message,
        })
    }
}
