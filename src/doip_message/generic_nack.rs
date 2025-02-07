use crate::{
    definitions::DOIP_GENERIC_NACK_LEN,
    error::{GenericNackError, PayloadError},
    header::{DoipPayload, PayloadType},
    message::NackCode,
};

/// The generic negative acknowledgement of a bad request.
///
/// This is found usually when a critical error occurs due to a bad DoIP packet
/// or an entity issue.
#[derive(Copy, Clone, Debug)]
pub struct GenericNack {
    /// Available negative acknowledgement codes
    pub nack_code: NackCode,
}

impl DoipPayload<'_> for GenericNack {
    fn payload_type(&self) -> PayloadType {
        PayloadType::GenericNack
    }

    fn to_bytes(&self, buffer: &mut [u8]) -> Result<usize, PayloadError> {
        let min_len = [self.nack_code as u8].len();

        if buffer.len() < min_len {
            return Err(PayloadError::BufferTooSmall);
        }

        let mut offset = 0;

        buffer[offset] = self.nack_code as u8;
        offset += 1;

        Ok(offset)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, PayloadError> {
        // Check that bytes have sufficient length
        let min_length = DOIP_GENERIC_NACK_LEN;

        if bytes.len() < min_length {
            return Err(PayloadError::GenericNackError(
                GenericNackError::InvalidLength,
            ));
        }

        let nack_code_offset = 0;
        let nack_code = match &bytes[nack_code_offset] {
            0x00 => NackCode::IncorrectPatternFormat,
            0x01 => NackCode::UnknownPayloadType,
            0x02 => NackCode::MessageTooLarge,
            0x03 => NackCode::OutOfMemory,
            0x04 => NackCode::InvalidPayloadLength,
            _ => {
                return Err(PayloadError::GenericNackError(
                    GenericNackError::InvalidNackCode,
                ))
            }
        };

        Ok(Self { nack_code })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        doip_message::generic_nack::GenericNack,
        error::{GenericNackError, PayloadError},
        header::{DoipPayload, PayloadType},
        message::NackCode,
    };

    const DEFAULT_NACK_CODE: NackCode = NackCode::IncorrectPatternFormat;

    #[test]
    fn test_payload_type() {
        let request = GenericNack {
            nack_code: DEFAULT_NACK_CODE,
        };
        assert_eq!(request.payload_type(), PayloadType::GenericNack);
    }

    #[test]
    fn test_to_bytes() {
        let mut buffer = [0; 1024];

        let request = GenericNack {
            nack_code: DEFAULT_NACK_CODE,
        };
        assert_eq!(request.to_bytes(&mut buffer), Ok(0));
    }

    #[test]
    fn test_from_bytes_too_short() {
        let request = [];
        let from_bytes = GenericNack::from_bytes(&request);

        assert!(
            from_bytes.is_err(),
            "Expected to receive an GenericNackError::InvalidLength."
        );

        let error = from_bytes.unwrap_err();

        assert_eq!(
            error,
            PayloadError::GenericNackError(GenericNackError::InvalidLength),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_invalid_nack_code() {
        let request = [0x05];
        let from_bytes = GenericNack::from_bytes(&request);

        assert!(
            from_bytes.is_err(),
            "Expected to receive an GenericNackError::InvalidNackCode."
        );

        let error = from_bytes.unwrap_err();

        assert_eq!(
            error,
            PayloadError::GenericNackError(GenericNackError::InvalidNackCode),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_ok() {
        let mut buffer = [0; 1024];
        let request = GenericNack {
            nack_code: DEFAULT_NACK_CODE,
        }
        .to_bytes(&mut buffer)
        .unwrap();
        let from_bytes = GenericNack::from_bytes(&buffer[..request]);

        assert!(
            from_bytes.is_ok(),
            "Expected GenericNack, recieved an Error."
        );
    }
}
