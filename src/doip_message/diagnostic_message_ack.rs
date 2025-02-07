use crate::{
    definitions::{
        DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN, DOIP_DIAG_MESSAGE_ACK_CODE_LEN,
    },
    error::{DiagnosticMessageAckError, PayloadError},
    header::{DoipPayload, PayloadType},
    message::DiagnosticAckCode,
};

/// Postive acknowledgement of a `DiagnosticMessage`.
///
/// Containing the source and target entity addresses, as well as the `DiagnosticAckCode`
/// for the `DiagnosticMessage` initially sent by the target entity.
#[derive(Copy, Clone, Debug)]
pub struct DiagnosticMessageAck {
    /// The source address of the responding DoIP Entity
    pub source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN],

    /// The target address of the requesting DoIP Entity
    pub target_address: [u8; DOIP_DIAG_COMMON_TARGET_LEN],

    /// The positive acknowledgement code
    pub ack_code: DiagnosticAckCode,
}

impl DoipPayload<'_> for DiagnosticMessageAck {
    fn payload_type(&self) -> PayloadType {
        PayloadType::DiagnosticMessageAck
    }

    fn to_bytes(&self, buffer: &mut [u8]) -> Result<usize, PayloadError> {
        let src_len = self.source_address.len();
        let tgt_len = self.target_address.len();
        let min_len = src_len + tgt_len + [self.ack_code as u8].len();

        if buffer.len() < min_len {
            return Err(PayloadError::BufferTooSmall);
        }

        let mut offset = 0;

        buffer[offset..offset + src_len].copy_from_slice(&self.source_address);
        offset += src_len;

        buffer[offset..offset + tgt_len].copy_from_slice(&self.target_address);
        offset += tgt_len;

        buffer[offset] = self.ack_code as u8;
        offset += 1;

        Ok(offset)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, PayloadError> {
        // Check that bytes have sufficient length
        let min_length = DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN;

        if bytes.len() < min_length {
            return Err(PayloadError::DiagnosticMessageAckError(
                DiagnosticMessageAckError::InvalidLength,
            ));
        }

        let source_address_offset = DOIP_DIAG_COMMON_SOURCE_LEN;
        let source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN] =
            match bytes[0..source_address_offset].try_into() {
                Ok(arr) => arr,
                Err(_) => {
                    return Err(PayloadError::DiagnosticMessageAckError(
                        DiagnosticMessageAckError::InvalidIndexRange,
                    ))
                }
            };

        let target_address_offset = source_address_offset + DOIP_DIAG_COMMON_TARGET_LEN;
        let target_address: [u8; DOIP_DIAG_COMMON_TARGET_LEN] =
            match bytes[source_address_offset..target_address_offset].try_into() {
                Ok(arr) => arr,
                Err(_) => {
                    return Err(PayloadError::DiagnosticMessageAckError(
                        DiagnosticMessageAckError::InvalidIndexRange,
                    ))
                }
            };

        let _ack_code_offset = target_address_offset + DOIP_DIAG_MESSAGE_ACK_CODE_LEN;
        let ack_code = match &bytes[target_address_offset] {
            0x00 => DiagnosticAckCode::Acknowledged,
            _ => {
                return Err(PayloadError::DiagnosticMessageAckError(
                    DiagnosticMessageAckError::InvalidAckCode,
                ))
            }
        };

        Ok(Self {
            source_address,
            target_address,
            ack_code,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        doip_message::diagnostic_message_ack::DiagnosticMessageAck,
        error::{DiagnosticMessageAckError, PayloadError},
        header::{DoipPayload, PayloadType},
        message::DiagnosticAckCode,
    };

    const DEFAULT_SOURCE_ADDRESS: [u8; 2] = [0x01, 0x02];
    const DEFAULT_TARGET_ADDRESS: [u8; 2] = [0x03, 0x04];
    const DEFAULT_ACK_CODE: DiagnosticAckCode = DiagnosticAckCode::Acknowledged;

    #[test]
    fn test_payload_type() {
        let request = DiagnosticMessageAck {
            source_address: DEFAULT_SOURCE_ADDRESS,
            target_address: DEFAULT_TARGET_ADDRESS,
            ack_code: DEFAULT_ACK_CODE,
        };
        assert_eq!(request.payload_type(), PayloadType::DiagnosticMessageAck);
    }

    #[test]
    fn test_to_bytes() {
        let request = DiagnosticMessageAck {
            source_address: DEFAULT_SOURCE_ADDRESS,
            target_address: DEFAULT_TARGET_ADDRESS,
            ack_code: DEFAULT_ACK_CODE,
        };
        let mut buffer = [0; 1024];
        assert_eq!(request.to_bytes(&mut buffer), Ok(5));
    }

    #[test]
    fn test_from_bytes_too_short() {
        let request = [0x01, 0x02, 0x03];
        let from_bytes = DiagnosticMessageAck::from_bytes(&request);

        assert!(
            from_bytes.is_err(),
            "Expected to receive an DiagnosticMessageAckError::InvalidLength."
        );

        let error = from_bytes.unwrap_err();

        assert_eq!(
            error,
            PayloadError::DiagnosticMessageAckError(DiagnosticMessageAckError::InvalidLength),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_invalid_ack_code() {
        let request = [0x01, 0x02, 0x03, 0x04, 0x01];
        let from_bytes = DiagnosticMessageAck::from_bytes(&request);

        assert!(
            from_bytes.is_err(),
            "Expected to receive an DiagnosticMessageAckError::InvalidAckCode."
        );

        let error = from_bytes.unwrap_err();

        assert_eq!(
            error,
            PayloadError::DiagnosticMessageAckError(DiagnosticMessageAckError::InvalidAckCode),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_ok() {
        let mut buffer = [0; 1024];
        let request = DiagnosticMessageAck {
            source_address: DEFAULT_SOURCE_ADDRESS,
            target_address: DEFAULT_TARGET_ADDRESS,
            ack_code: DEFAULT_ACK_CODE,
        }
        .to_bytes(&mut buffer)
        .unwrap();
        let from_bytes = DiagnosticMessageAck::from_bytes(&buffer[..request]);

        assert!(
            from_bytes.is_ok(),
            "Expected DiagnosticMessageAck, recieved an Error."
        );
    }
}
