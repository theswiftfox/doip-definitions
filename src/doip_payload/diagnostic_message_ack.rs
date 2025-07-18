use crate::{
    definitions::{DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN},
    error::{Error, Result},
    payload::DiagnosticAckCode,
};

/// Postive acknowledgement of a `DiagnosticMessage`.
///
/// Containing the source and target entity addresses, as well as the `DiagnosticAckCode`
/// for the `DiagnosticMessage` initially sent by the target entity.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DiagnosticMessageAck {
    /// The source address of the responding `DoIP` Entity
    pub source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN],

    /// The target address of the requesting `DoIP` Entity
    pub target_address: [u8; DOIP_DIAG_COMMON_TARGET_LEN],

    /// The positive acknowledgement code
    pub ack_code: DiagnosticAckCode,
}

impl From<DiagnosticMessageAck>
    for [u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]
{
    fn from(value: DiagnosticMessageAck) -> Self {
        let mut buffer = [0u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1];

        let mut offset = 0;

        buffer[offset..offset + DOIP_DIAG_COMMON_SOURCE_LEN].copy_from_slice(&value.source_address);
        offset += DOIP_DIAG_COMMON_SOURCE_LEN;

        buffer[offset..offset + DOIP_DIAG_COMMON_TARGET_LEN].copy_from_slice(&value.target_address);
        offset += DOIP_DIAG_COMMON_TARGET_LEN;

        buffer[offset] = value.ack_code.into();

        buffer
    }
}

impl TryFrom<&[u8]> for DiagnosticMessageAck {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let mut offset = 0;

        let source_address = value
            .get(offset..DOIP_DIAG_COMMON_SOURCE_LEN)
            .ok_or(Error::OutOfBounds {
                source: "DiagnosticMessageAck",
                variable: "Source Address",
            })?
            .try_into()?;

        offset += DOIP_DIAG_COMMON_SOURCE_LEN;

        let target_address = value
            .get(offset..offset + DOIP_DIAG_COMMON_TARGET_LEN)
            .ok_or(Error::OutOfBounds {
                source: "DiagnosticMessageAck",
                variable: "Target Address",
            })?
            .try_into()?;

        offset += DOIP_DIAG_COMMON_TARGET_LEN;

        let ack_code = value
            .get(offset)
            .ok_or(Error::OutOfBounds {
                source: "DiagnosticMessageAck",
                variable: "Ack Code",
            })?
            .try_into()?;

        Ok(DiagnosticMessageAck {
            source_address,
            target_address,
            ack_code,
        })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_ack_size_is_correct() {
        let sizea = size_of::<[u8; crate::definitions::DOIP_DIAG_COMMON_SOURCE_LEN]>();
        assert_eq!(sizea, crate::definitions::DOIP_DIAG_COMMON_SOURCE_LEN);
        let sizeb = size_of::<crate::payload::DiagnosticAckCode>();
        assert_eq!(sizeb, size_of::<u8>());
    }
}
