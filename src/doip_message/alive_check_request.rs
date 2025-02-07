use crate::{
    error::PayloadError,
    header::{DoipPayload, PayloadType},
};

/// Checks the TCP Socket is still alive
///
/// Sent with no payload, the `AliveCheckRequest` is utilised to maintain a connection
/// to a TCP socket or to check the status of one.
#[derive(Copy, Clone, Debug)]
pub struct AliveCheckRequest {}

impl DoipPayload<'_> for AliveCheckRequest {
    fn payload_type(&self) -> PayloadType {
        PayloadType::AliveCheckRequest
    }

    fn to_bytes(&self, buffer: &mut [u8]) -> Result<usize, PayloadError> {
        let _ = buffer;
        Ok(0)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, PayloadError> {
        let _ = bytes;
        Ok(AliveCheckRequest {})
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        doip_message::alive_check_request::AliveCheckRequest,
        header::{DoipPayload, PayloadType},
    };

    #[test]
    fn test_payload_type() {
        let request = AliveCheckRequest {};

        assert_eq!(request.payload_type(), PayloadType::AliveCheckRequest);
    }

    #[test]
    fn test_to_bytes() {
        let request = AliveCheckRequest {};
        let mut buffer = [0; 1024];
        assert_eq!(request.to_bytes(&mut buffer), Ok(0));
    }

    #[test]
    fn test_from_bytes_ok() {
        let mut buffer = [0; 1024];
        let bytes = AliveCheckRequest {}.to_bytes(&mut buffer).unwrap();
        let request = AliveCheckRequest::from_bytes(&buffer[..bytes]);

        assert!(
            request.is_ok(),
            "Expected AliveCheckRequest, recieved an Error."
        );
    }
}
