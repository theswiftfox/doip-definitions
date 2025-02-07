use crate::{
    definitions::{DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN},
    error::{DiagnosticMessageError, PayloadError},
    header::{DoipPayload, PayloadType},
};

/// A UDS Message to a specific target address.
///
/// `DiagnosticMessage` is the most utilised payload type due to the amount of actions
/// a diagnostic tester can do using the UDS protocol. This crate will not handle the UDS
/// protocol however, one will be developed to enhance developer tooling.
#[derive(Clone, Debug)]
pub struct DiagnosticMessage<'a> {
    /// The source address of the responding DoIP Entity
    pub source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN],

    /// The target address of the requesting DoIP Entity
    pub target_address: [u8; DOIP_DIAG_COMMON_TARGET_LEN],

    /// Message containing the UDS protocol message
    pub message: &'a [u8],
}

impl<'a> DoipPayload<'a> for DiagnosticMessage<'a> {
    fn payload_type(&self) -> PayloadType {
        PayloadType::DiagnosticMessage
    }

    fn to_bytes(&self, buffer: &mut [u8]) -> Result<usize, PayloadError> {
        let src_len = self.source_address.len();
        let tgt_len = self.target_address.len();
        let msg_len = self.message.len();
        let min_len = src_len + tgt_len + msg_len;

        if buffer.len() < min_len {
            return Err(PayloadError::BufferTooSmall);
        }

        let mut offset = 0;

        buffer[offset..offset + src_len].copy_from_slice(&self.source_address);
        offset += src_len;

        buffer[offset..offset + tgt_len].copy_from_slice(&self.target_address);
        offset += tgt_len;

        buffer[offset..offset + msg_len].copy_from_slice(&self.message);
        offset += msg_len;

        Ok(offset)
    }

    fn from_bytes(bytes: &'a [u8]) -> Result<Self, PayloadError> {
        let min_length = DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN;

        if bytes.len() < min_length {
            return Err(PayloadError::DiagnosticMessageError(
                DiagnosticMessageError::InvalidLength,
            ));
        }

        let source_address_offset = DOIP_DIAG_COMMON_SOURCE_LEN;
        let source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN] =
            bytes[0..source_address_offset].try_into().map_err(|_| {
                PayloadError::DiagnosticMessageError(DiagnosticMessageError::InvalidIndexRange)
            })?;

        let target_address_offset = source_address_offset + DOIP_DIAG_COMMON_TARGET_LEN;
        let target_address: [u8; DOIP_DIAG_COMMON_TARGET_LEN] = bytes
            [source_address_offset..target_address_offset]
            .try_into()
            .map_err(|_| {
                PayloadError::DiagnosticMessageError(DiagnosticMessageError::InvalidIndexRange)
            })?;

        let message = &bytes[target_address_offset..];

        Ok(Self {
            source_address,
            target_address,
            message,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        doip_message::diagnostic_message::DiagnosticMessage,
        error::{DiagnosticMessageError, PayloadError},
        header::{DoipPayload, PayloadType},
    };

    const DEFAULT_SOURCE_ADDRESS: [u8; 2] = [0x01, 0x02];
    const DEFAULT_TARGET_ADDRESS: [u8; 2] = [0x03, 0x04];

    #[test]
    fn test_payload_type() {
        let request = DiagnosticMessage {
            source_address: DEFAULT_SOURCE_ADDRESS,
            target_address: DEFAULT_TARGET_ADDRESS,
            message: &[0x05, 0x06, 0x07, 0x08],
        };
        assert_eq!(request.payload_type(), PayloadType::DiagnosticMessage);
    }

    #[test]
    fn test_to_bytes() {
        let request = DiagnosticMessage {
            source_address: DEFAULT_SOURCE_ADDRESS,
            target_address: DEFAULT_TARGET_ADDRESS,
            message: &[0x05, 0x06, 0x07, 0x08],
        };

        let mut buffer = [0; 1024];

        assert_eq!(request.to_bytes(&mut buffer), Ok(8));
    }

    #[test]
    fn test_from_bytes_too_short() {
        let request = [0x01, 0x02, 0x03];
        let from_bytes = DiagnosticMessage::from_bytes(&request);

        assert!(
            from_bytes.is_err(),
            "Expected to receive an DiagnosticMessage::InvalidLength."
        );

        let error = from_bytes.unwrap_err();

        assert_eq!(
            error,
            PayloadError::DiagnosticMessageError(DiagnosticMessageError::InvalidLength),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_ok() {
        let mut buffer = [0; 1024];
        let request = DiagnosticMessage {
            source_address: DEFAULT_SOURCE_ADDRESS,
            target_address: DEFAULT_TARGET_ADDRESS,
            message: &[0x05, 0x06, 0x07, 0x08],
        }
        .to_bytes(&mut buffer)
        .unwrap();
        let from_bytes = DiagnosticMessage::from_bytes(&buffer[..request]);

        assert!(
            from_bytes.is_ok(),
            "Expected DiagnosticMessage, recieved an Error."
        );
    }
}
