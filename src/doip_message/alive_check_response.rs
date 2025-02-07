use crate::{
    definitions::DOIP_DIAG_COMMON_SOURCE_LEN,
    error::{AliveCheckResponseError, PayloadError},
    header::{DoipPayload, PayloadType},
};

/// Confirmation of the `AliveCheckRequest`.
///
/// The typical response from an `AliveCheckRequest`.
#[derive(Copy, Clone, Debug)]
pub struct AliveCheckResponse {
    /// The source address of the responding DoIP Entity
    pub source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN],
}

impl DoipPayload<'_> for AliveCheckResponse {
    fn payload_type(&self) -> PayloadType {
        PayloadType::AliveCheckResponse
    }

    fn to_bytes(&self, buffer: &mut [u8]) -> Result<usize, PayloadError> {
        if buffer.len() < 4 {
            return Err(PayloadError::BufferTooSmall);
        }

        buffer[..4].copy_from_slice(&self.source_address);

        Ok(self.source_address.len())
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, PayloadError> {
        // Check that bytes have sufficient length
        let min_length = DOIP_DIAG_COMMON_SOURCE_LEN;

        if bytes.len() < min_length {
            return Err(PayloadError::AliveCheckResponseParseError(
                AliveCheckResponseError::InvalidLength,
            ));
        }

        let source_address_offset = DOIP_DIAG_COMMON_SOURCE_LEN;
        let source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN] =
            match bytes[0..source_address_offset].try_into() {
                Ok(array) => array,
                Err(_) => {
                    return Err(PayloadError::AliveCheckResponseParseError(
                        AliveCheckResponseError::InvalidIndexRange,
                    ))
                }
            };

        Ok(Self { source_address })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        doip_message::alive_check_response::{AliveCheckResponse, AliveCheckResponseError},
        error::PayloadError,
        header::{DoipPayload, PayloadType},
    };

    const SOURCE_ADDRESS: [u8; 2] = [0x01, 0x02];
    const INVALID_LENGTH: [u8; 1] = [0x03];

    #[test]
    fn test_payload_type() {
        let request = AliveCheckResponse {
            source_address: SOURCE_ADDRESS,
        };
        assert_eq!(request.payload_type(), PayloadType::AliveCheckResponse);
    }

    #[test]
    fn test_to_bytes() {
        let request = AliveCheckResponse {
            source_address: SOURCE_ADDRESS,
        };
        let mut buffer = [0; 1024];
        assert_eq!(request.to_bytes(&mut buffer), Ok(SOURCE_ADDRESS.len()));
    }

    #[test]
    fn test_from_bytes_invalid_length() {
        let bytes = INVALID_LENGTH;
        let request = AliveCheckResponse::from_bytes(&bytes);

        assert!(
            request.is_err(),
            "Expected to receive an AliveCheckResponseParse::InvalidLength."
        );

        let error = request.unwrap_err();

        assert_eq!(
            error,
            PayloadError::AliveCheckResponseParseError(AliveCheckResponseError::InvalidLength),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_ok() {
        let buffer = [0; 1024];

        let request = AliveCheckResponse::from_bytes(&buffer);

        assert!(
            request.is_ok(),
            "Expected AliveCheckResponse, recieved an Error."
        );
    }
}
