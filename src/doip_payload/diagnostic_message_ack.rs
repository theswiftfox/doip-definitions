use crate::{
    definitions::{DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN},
    payload::DiagnosticAckCode,
};

/// Postive acknowledgement of a `DiagnosticMessage`.
///
/// Containing the source and target entity addresses, as well as the `DiagnosticAckCode`
/// for the `DiagnosticMessage` initially sent by the target entity.
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
    fn from(diagnostic_message_ack: DiagnosticMessageAck) -> Self {
        let source_address = diagnostic_message_ack.source_address;
        let target_address = diagnostic_message_ack.target_address;
        let ack_code = [u8::from(diagnostic_message_ack.ack_code)];

        let mut buffer = [0; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1];
        let mut offset = 0;

        buffer[offset..offset + DOIP_DIAG_COMMON_SOURCE_LEN].copy_from_slice(&source_address);
        offset += DOIP_DIAG_COMMON_SOURCE_LEN;

        buffer[offset..offset + DOIP_DIAG_COMMON_TARGET_LEN].copy_from_slice(&target_address);
        offset += DOIP_DIAG_COMMON_TARGET_LEN;

        buffer[offset] = ack_code[0];

        buffer
    }
}

impl TryFrom<[u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]>
    for DiagnosticMessageAck
{
    type Error = &'static str;

    fn try_from(
        value: [u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1],
    ) -> Result<Self, Self::Error> {
        let (source_slice, rest) = value.split_at(DOIP_DIAG_COMMON_SOURCE_LEN);
        let (target_slice, ack_bytes) = rest.split_at(DOIP_DIAG_COMMON_TARGET_LEN);

        let mut source_address = [0u8; DOIP_DIAG_COMMON_SOURCE_LEN];
        source_address.copy_from_slice(source_slice);

        let mut target_address = [0u8; DOIP_DIAG_COMMON_TARGET_LEN];
        target_address.copy_from_slice(target_slice);

        let ack_code = DiagnosticAckCode::try_from(ack_bytes[0])?;

        Ok(DiagnosticMessageAck {
            source_address,
            target_address,
            ack_code,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        definitions::{DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN},
        payload::DiagnosticAckCode,
    };

    use super::DiagnosticMessageAck;

    #[test]
    fn test_try_from_bytes() {
        for a in u8::MIN..=u8::MAX {
            let message_ack = DiagnosticMessageAck::try_from([0x12, 0x34, 0x56, 0x78, a]);

            match a {
                0x00 => assert_eq!(
                    message_ack.unwrap(),
                    DiagnosticMessageAck {
                        source_address: [0x12, 0x34],
                        target_address: [0x56, 0x78],
                        ack_code: DiagnosticAckCode::Acknowledged
                    }
                ),
                _ => assert_eq!(message_ack.unwrap_err(), "Invalid DiagnosticAckCode."),
            };
        }
    }

    #[test]
    fn test_from_message_ack() {
        let a: u32 = 0x01234567;
        let bytes = a.to_be_bytes();
        let ack_code: u8 = 0x00;
        let message_ack = DiagnosticMessageAck {
            source_address: [bytes[0], bytes[1]],
            target_address: [bytes[2], bytes[3]],
            ack_code: DiagnosticAckCode::try_from(ack_code).unwrap(),
        };

        let message_ack_bytes =
            <[u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]>::from(
                message_ack,
            );

        assert_eq!(
            message_ack_bytes,
            [bytes[0], bytes[1], bytes[2], bytes[3], ack_code]
        );
    }
}
