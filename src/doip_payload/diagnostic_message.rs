use crate::definitions::{DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN};

/// A UDS Message to a specific target address.
///
/// `DiagnosticMessage` is the most utilised payload type due to the amount of actions
/// a diagnostic tester can do using the UDS protocol. This crate will not handle the UDS
/// protocol however, one will be developed to enhance developer tooling.
#[derive(Clone, Debug, PartialEq)]
pub struct DiagnosticMessage<'a> {
    /// The source address of the responding `DoIP` Entity
    pub source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN],

    /// The target address of the requesting `DoIP` Entity
    pub target_address: [u8; DOIP_DIAG_COMMON_TARGET_LEN],

    /// Message containing the UDS protocol message
    pub message: &'a [u8],
}

impl<'a, const N: usize> TryFrom<DiagnosticMessage<'a>> for [u8; N] {
    type Error = &'static str;

    fn try_from(diagnostic_message: DiagnosticMessage<'a>) -> Result<Self, Self::Error> {
        let message_len = diagnostic_message.message.len();

        if N < DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + message_len {
            return Err("Invalid array size");
        }

        let mut buffer = [0u8; N];
        let mut offset = 0;

        buffer[offset..offset + DOIP_DIAG_COMMON_SOURCE_LEN]
            .copy_from_slice(&diagnostic_message.source_address);
        offset += DOIP_DIAG_COMMON_SOURCE_LEN;

        buffer[offset..offset + DOIP_DIAG_COMMON_TARGET_LEN]
            .copy_from_slice(&diagnostic_message.target_address);
        offset += DOIP_DIAG_COMMON_TARGET_LEN;

        buffer[offset..].copy_from_slice(diagnostic_message.message);

        Ok(buffer)
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

        let mut source_address = [0u8; DOIP_DIAG_COMMON_SOURCE_LEN];
        source_address.copy_from_slice(source_slice);

        let mut target_address = [0u8; DOIP_DIAG_COMMON_TARGET_LEN];
        target_address.copy_from_slice(target_slice);

        Ok(DiagnosticMessage {
            source_address,
            target_address,
            message,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::DiagnosticMessage;

    #[test]
    fn test_try_from_bytes() {
        let u: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05];
        let diag_msg = DiagnosticMessage::try_from(u);

        assert_eq!(
            diag_msg.unwrap(),
            DiagnosticMessage {
                source_address: [0x01, 0x02],
                target_address: [0x03, 0x04],
                message: &[0x05]
            }
        )
    }

    #[test]
    fn test_try_from_bytes_too_short() {
        let u: &[u8] = &[0x00];
        let diag_msg = DiagnosticMessage::try_from(u);

        assert_eq!(
            diag_msg.unwrap_err(),
            "Input slice too short to be a valid DiagnosticMessage"
        )
    }

    #[test]
    fn test_try_from_message() {
        let diag_msg = DiagnosticMessage {
            source_address: [0x01, 0x02],
            target_address: [0x03, 0x04],
            message: &[0x05],
        };

        const N: usize = 5;

        let diag_msg_bytes = <[u8; N]>::try_from(diag_msg);
        assert_eq!(diag_msg_bytes.unwrap(), [0x01, 0x02, 0x03, 0x04, 0x05]);
    }

    #[test]
    fn test_try_from_message_invalid_size() {
        let diag_msg = DiagnosticMessage {
            source_address: [0x01, 0x02],
            target_address: [0x03, 0x04],
            message: &[0x05],
        };

        const N: usize = 4;

        let diag_msg_bytes = <[u8; N]>::try_from(diag_msg);
        assert_eq!(diag_msg_bytes.unwrap_err(), "Invalid array size");
    }

    #[test]
    fn test_try_from_message_empty() {
        let diag_msg = DiagnosticMessage {
            source_address: [0x01, 0x02],
            target_address: [0x03, 0x04],
            message: &[],
        };

        const N: usize = 4;

        let diag_msg_bytes = <[u8; N]>::try_from(diag_msg);
        assert_eq!(diag_msg_bytes.unwrap(), [0x01, 0x02, 0x03, 0x04]);
    }
}
