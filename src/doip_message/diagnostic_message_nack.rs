use crate::{
    definitions::{
        DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN, DOIP_DIAG_MESSAGE_NACK_CODE_LEN,
    },
    error::{DiagnosticMessageNackError, PayloadError},
    header::{DoipPayload, PayloadType},
    message::DiagnosticNackCode,
};

/// Negative acknowledgement of a `DiagnosticMessage`.
///
/// Containing the source and target entity addresses, as well as the `DiagnosticNackCode`
/// for the `DiagnosticMessage` initially sent by the target entity.
#[derive(Copy, Clone, Debug)]
pub struct DiagnosticMessageNack {
    /// The source address of the responding DoIP Entity
    pub source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN],

    /// The target address of the requesting DoIP Entity
    pub target_address: [u8; DOIP_DIAG_COMMON_TARGET_LEN],

    /// The negative acknowledgement code
    pub nack_code: DiagnosticNackCode,
}

impl DoipPayload<'_> for DiagnosticMessageNack {
    fn payload_type(&self) -> PayloadType {
        PayloadType::DiagnosticMessageNack
    }

    fn to_bytes(&self, buffer: &mut [u8]) -> Result<usize, PayloadError> {
        let src_len = self.source_address.len();
        let tgt_len = self.target_address.len();
        let min_len = [self.nack_code as u8].len() + tgt_len + src_len;

        if buffer.len() < min_len {
            return Err(PayloadError::BufferTooSmall);
        }

        let mut offset = 0;

        buffer[offset..offset + src_len].copy_from_slice(&self.source_address);
        offset += src_len;

        buffer[offset..offset + tgt_len].copy_from_slice(&self.target_address);
        offset += tgt_len;

        buffer[offset] = self.nack_code as u8;
        offset += 1;

        Ok(offset)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, PayloadError> {
        // Check that bytes have sufficient length
        let min_length = DOIP_DIAG_COMMON_SOURCE_LEN
            + DOIP_DIAG_COMMON_TARGET_LEN
            + DOIP_DIAG_MESSAGE_NACK_CODE_LEN;

        if bytes.len() < min_length {
            return Err(PayloadError::DiagnosticMessageNackParseError(
                DiagnosticMessageNackError::InvalidLength,
            ));
        }

        let source_address_offset = DOIP_DIAG_COMMON_SOURCE_LEN;
        let source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN] =
            match bytes[0..source_address_offset].try_into() {
                Ok(array) => array,
                Err(_) => {
                    return Err(PayloadError::DiagnosticMessageNackParseError(
                        DiagnosticMessageNackError::InvalidIndexRange,
                    ))
                }
            };

        let target_address_offset = source_address_offset + DOIP_DIAG_COMMON_TARGET_LEN;
        let target_address: [u8; DOIP_DIAG_COMMON_TARGET_LEN] =
            match bytes[source_address_offset..target_address_offset].try_into() {
                Ok(array) => array,
                Err(_) => {
                    return Err(PayloadError::DiagnosticMessageNackParseError(
                        DiagnosticMessageNackError::InvalidIndexRange,
                    ))
                }
            };

        let nack_code_offset = target_address_offset;
        let nack_code = match &bytes[nack_code_offset] {
            0x00 => DiagnosticNackCode::ReservedByIso13400_00,
            0x01 => DiagnosticNackCode::ReservedByIso13400_01,
            0x02 => DiagnosticNackCode::InvalidSourceAddress,
            0x03 => DiagnosticNackCode::UnknownTargetAddress,
            0x04 => DiagnosticNackCode::DiagnosticMessageTooLarge,
            0x05 => DiagnosticNackCode::OutOfMemory,
            0x06 => DiagnosticNackCode::TargetUnreachable,
            0x07 => DiagnosticNackCode::UnknownNetwork,
            0x08 => DiagnosticNackCode::TransportProtocolError,
            _ => {
                return Err(PayloadError::DiagnosticMessageNackParseError(
                    DiagnosticMessageNackError::InvalidNackCode,
                ))
            }
        };

        Ok(Self {
            source_address,
            target_address,
            nack_code,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        doip_message::diagnostic_message_nack::DiagnosticMessageNack,
        error::{DiagnosticMessageNackError, PayloadError},
        header::{DoipPayload, PayloadType},
        message::DiagnosticNackCode,
    };

    const DEFAULT_SOURCE_ADDRESS: [u8; 2] = [0x01, 0x02];
    const DEFAULT_TARGET_ADDRESS: [u8; 2] = [0x03, 0x04];
    const DEFAULT_NACK_CODE: DiagnosticNackCode = DiagnosticNackCode::ReservedByIso13400_00;

    #[test]
    fn test_payload_type() {
        let request = DiagnosticMessageNack {
            source_address: DEFAULT_SOURCE_ADDRESS,
            target_address: DEFAULT_TARGET_ADDRESS,
            nack_code: DEFAULT_NACK_CODE,
        };
        assert_eq!(request.payload_type(), PayloadType::DiagnosticMessageNack);
    }

    #[test]
    fn test_to_bytes() {
        let request = DiagnosticMessageNack {
            source_address: DEFAULT_SOURCE_ADDRESS,
            target_address: DEFAULT_TARGET_ADDRESS,
            nack_code: DEFAULT_NACK_CODE,
        };

        let mut buffer = [0; 1024];
        assert_eq!(request.to_bytes(&mut buffer), Ok(5));
    }

    #[test]
    fn test_from_bytes_too_short() {
        let request = [0x01, 0x02, 0x03];
        let from_bytes = DiagnosticMessageNack::from_bytes(&request);

        assert!(
            from_bytes.is_err(),
            "Expected to receive an DiagnosticMessageNackParseError::InvalidLength."
        );

        let error = from_bytes.unwrap_err();

        assert_eq!(
            error,
            PayloadError::DiagnosticMessageNackParseError(
                DiagnosticMessageNackError::InvalidLength
            ),
            "Unexpected error message: {}",
            error
        );
    }
    #[test]
    fn test_from_bytes_invalid_nack_code() {
        let request = [0x01, 0x02, 0x03, 0x04, 0x09];
        let from_bytes = DiagnosticMessageNack::from_bytes(&request);

        assert!(
            from_bytes.is_err(),
            "Expected to receive an DiagnosticMessageNackParseError::InvalidAckCode."
        );

        let error = from_bytes.unwrap_err();

        assert_eq!(
            error,
            PayloadError::DiagnosticMessageNackParseError(
                DiagnosticMessageNackError::InvalidNackCode
            ),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_ok() {
        let mut buffer = [0; 1024];
        let request = DiagnosticMessageNack {
            source_address: DEFAULT_SOURCE_ADDRESS,
            target_address: DEFAULT_TARGET_ADDRESS,
            nack_code: DEFAULT_NACK_CODE,
        }
        .to_bytes(&mut buffer)
        .unwrap();
        let from_bytes = DiagnosticMessageNack::from_bytes(&buffer[..request]);

        assert!(
            from_bytes.is_ok(),
            "Expected DiagnosticMessageNack, recieved an Error."
        );
    }
}
