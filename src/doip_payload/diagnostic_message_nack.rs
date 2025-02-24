use crate::{
    definitions::{DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN},
    payload::DiagnosticNackCode,
};

/// Negative acknowledgement of a `DiagnosticMessage`.
///
/// Containing the source and target entity addresses, as well as the `DiagnosticNackCode`
/// for the `DiagnosticMessage` initially sent by the target entity.
#[derive(Copy, Clone, Debug, PartialEq)]
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

        let mut source_address = [0u8; DOIP_DIAG_COMMON_SOURCE_LEN];
        source_address.copy_from_slice(source_slice);

        let mut target_address = [0u8; DOIP_DIAG_COMMON_TARGET_LEN];
        target_address.copy_from_slice(target_slice);

        let nack_code = DiagnosticNackCode::try_from(nack_bytes[0])?;

        Ok(DiagnosticMessageNack {
            source_address,
            target_address,
            nack_code,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        definitions::{DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN},
        payload::DiagnosticNackCode,
    };

    use super::DiagnosticMessageNack;

    #[test]
    fn test_try_from_bytes() {
        for a in u8::MIN..=u8::MAX {
            let message_nack = DiagnosticMessageNack::try_from([0x12, 0x34, 0x56, 0x78, a]);

            match a {
                0x00 => assert_eq!(
                    message_nack.unwrap(),
                    DiagnosticMessageNack {
                        source_address: [0x12, 0x34],
                        target_address: [0x56, 0x78],
                        nack_code: DiagnosticNackCode::ReservedByIso13400_00,
                    }
                ),
                0x01 => assert_eq!(
                    message_nack.unwrap(),
                    DiagnosticMessageNack {
                        source_address: [0x12, 0x34],
                        target_address: [0x56, 0x78],
                        nack_code: DiagnosticNackCode::ReservedByIso13400_01,
                    }
                ),
                0x02 => assert_eq!(
                    message_nack.unwrap(),
                    DiagnosticMessageNack {
                        source_address: [0x12, 0x34],
                        target_address: [0x56, 0x78],
                        nack_code: DiagnosticNackCode::InvalidSourceAddress,
                    }
                ),
                0x03 => assert_eq!(
                    message_nack.unwrap(),
                    DiagnosticMessageNack {
                        source_address: [0x12, 0x34],
                        target_address: [0x56, 0x78],
                        nack_code: DiagnosticNackCode::UnknownTargetAddress,
                    }
                ),
                0x04 => assert_eq!(
                    message_nack.unwrap(),
                    DiagnosticMessageNack {
                        source_address: [0x12, 0x34],
                        target_address: [0x56, 0x78],
                        nack_code: DiagnosticNackCode::DiagnosticMessageTooLarge,
                    }
                ),
                0x05 => assert_eq!(
                    message_nack.unwrap(),
                    DiagnosticMessageNack {
                        source_address: [0x12, 0x34],
                        target_address: [0x56, 0x78],
                        nack_code: DiagnosticNackCode::OutOfMemory,
                    }
                ),
                0x06 => assert_eq!(
                    message_nack.unwrap(),
                    DiagnosticMessageNack {
                        source_address: [0x12, 0x34],
                        target_address: [0x56, 0x78],
                        nack_code: DiagnosticNackCode::TargetUnreachable,
                    }
                ),
                0x07 => assert_eq!(
                    message_nack.unwrap(),
                    DiagnosticMessageNack {
                        source_address: [0x12, 0x34],
                        target_address: [0x56, 0x78],
                        nack_code: DiagnosticNackCode::UnknownNetwork,
                    }
                ),
                0x08 => assert_eq!(
                    message_nack.unwrap(),
                    DiagnosticMessageNack {
                        source_address: [0x12, 0x34],
                        target_address: [0x56, 0x78],
                        nack_code: DiagnosticNackCode::TransportProtocolError,
                    }
                ),
                _ => assert_eq!(message_nack.unwrap_err(), "Invalid DiagnosticNackCode."),
            };
        }
    }

    #[test]
    fn test_from_message_nack() {
        let a: u32 = 0x01234567;
        let bytes = a.to_be_bytes();
        let nack_code: u8 = 0x00;
        let message_ack = DiagnosticMessageNack {
            source_address: [bytes[0], bytes[1]],
            target_address: [bytes[2], bytes[3]],
            nack_code: DiagnosticNackCode::try_from(nack_code).unwrap(),
        };

        let message_ack_bytes =
            <[u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]>::from(
                message_ack,
            );

        assert_eq!(
            message_ack_bytes,
            [bytes[0], bytes[1], bytes[2], bytes[3], nack_code]
        );
    }
}
