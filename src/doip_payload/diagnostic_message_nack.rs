use crate::{
    definitions::{DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN},
    payload::DiagnosticNackCode,
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

impl From<DiagnosticMessageNack>
    for [u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]
{
    fn from(diagnostic_message_nack: DiagnosticMessageNack) -> Self {
        let source_address = diagnostic_message_nack.source_address;
        let target_address = diagnostic_message_nack.target_address;
        let nack_code = [u8::from(diagnostic_message_nack.nack_code)];

        let mut buffer = [0; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1];
        let mut offset = 0;

        buffer[offset..offset + DOIP_DIAG_COMMON_SOURCE_LEN].copy_from_slice(&source_address);
        offset += DOIP_DIAG_COMMON_SOURCE_LEN;

        buffer[offset..offset + DOIP_DIAG_COMMON_TARGET_LEN].copy_from_slice(&target_address);
        offset += DOIP_DIAG_COMMON_TARGET_LEN;

        buffer[offset] = nack_code[0];

        buffer
    }
}

impl TryFrom<[u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]>
    for DiagnosticMessageNack
{
    type Error = &'static str;

    fn try_from(
        value: [u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1],
    ) -> Result<Self, Self::Error> {
        let (source_slice, rest) = value.split_at(DOIP_DIAG_COMMON_SOURCE_LEN);
        let (target_slice, nack_bytes) = rest.split_at(DOIP_DIAG_COMMON_TARGET_LEN);

        let source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN] = source_slice
            .try_into()
            .map_err(|_| "Invalid source address length")?;

        let target_address: [u8; DOIP_DIAG_COMMON_TARGET_LEN] = target_slice
            .try_into()
            .map_err(|_| "Invalid target address length")?;

        let nack_code =
            DiagnosticNackCode::try_from(nack_bytes[0]).map_err(|_| "Invalid nack code")?;

        Ok(DiagnosticMessageNack {
            source_address,
            target_address,
            nack_code,
        })
    }
}
