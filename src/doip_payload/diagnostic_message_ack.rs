use crate::{
    definitions::{DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN},
    doip_payload::SizedDoipPayload,
    error::{Error, Result},
    payload::DiagnosticAckCode,
};

/// Postive acknowledgement of a `DiagnosticMessage`.
///
/// Containing the source and target entity addresses, as well as the `DiagnosticAckCode`
/// for the `DiagnosticMessage` initially sent by the target entity.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass)]
#[derive(Clone, Debug, PartialEq)]
#[repr(C)]
pub struct DiagnosticMessageAck {
    /// The source address of the responding `DoIP` Entity
    pub source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN],

    /// The target address of the requesting `DoIP` Entity
    pub target_address: [u8; DOIP_DIAG_COMMON_TARGET_LEN],

    /// The positive acknowledgement code
    pub ack_code: DiagnosticAckCode,

    /// The previous message that was acknowledged
    pub previous_message: Vec<u8>,
}

impl SizedDoipPayload for DiagnosticMessageAck {
    /// Returns the size of the `DiagnosticMessageAck` payload in bytes.
    fn size_of(&self) -> usize {
        DOIP_DIAG_COMMON_SOURCE_LEN
            + DOIP_DIAG_COMMON_TARGET_LEN
            + std::mem::size_of::<DiagnosticAckCode>()
            + self.previous_message.len()
    }
}

impl From<DiagnosticMessageAck> for Vec<u8> {
    fn from(value: DiagnosticMessageAck) -> Self {
        let mut buffer = Vec::with_capacity(
            DOIP_DIAG_COMMON_SOURCE_LEN
                + DOIP_DIAG_COMMON_TARGET_LEN
                + 1
                + value.previous_message.len(),
        );

        buffer.extend_from_slice(&value.source_address);
        buffer.extend_from_slice(&value.target_address);
        buffer.push(value.ack_code.into());
        buffer.extend_from_slice(&value.previous_message);

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

        offset += 1;

        let previous_message = value[offset..].to_vec();

        Ok(DiagnosticMessageAck {
            source_address,
            target_address,
            ack_code,
            previous_message,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{
        header::ProtocolVersion,
        payload::{DiagnosticMessageAck, DoipPayload},
    };

    #[test]
    fn test_builder() {
        let msg = crate::builder::DoipMessageBuilder::new()
            .protocol_version(ProtocolVersion::Iso13400_2012)
            .payload(DoipPayload::DiagnosticMessageAck(DiagnosticMessageAck {
                source_address: [0; crate::definitions::DOIP_DIAG_COMMON_SOURCE_LEN],
                target_address: [0; crate::definitions::DOIP_DIAG_COMMON_TARGET_LEN],
                ack_code: crate::payload::DiagnosticAckCode::Acknowledged,
                previous_message: vec![0; 10], // Example previous message
            }))
            .build();

        assert_eq!(msg.header.protocol_version, ProtocolVersion::Iso13400_2012);
        assert_eq!(
            msg.header.payload_type,
            crate::header::PayloadType::DiagnosticMessageAck
        );
        assert_eq!(msg.header.payload_length, 15);
    }
}
