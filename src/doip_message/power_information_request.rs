use crate::{
    error::PayloadError,
    header::{DoipPayload, PayloadType},
};

/// Requests the power mode status of a DoIP Entity.
#[derive(Copy, Clone, Debug)]
pub struct PowerInformationRequest {}

impl DoipPayload<'_> for PowerInformationRequest {
    fn payload_type(&self) -> PayloadType {
        PayloadType::PowerInformationRequest
    }

    fn to_bytes(&self, buffer: &mut [u8]) -> Result<usize, PayloadError> {
        let _ = buffer;
        Ok(0)
    }

    fn from_bytes(_bytes: &[u8]) -> Result<Self, PayloadError> {
        Ok(Self {})
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        doip_message::power_information_request::PowerInformationRequest,
        header::{DoipPayload, PayloadType},
    };

    #[test]
    fn test_payload_type() {
        let request = PowerInformationRequest {};
        assert_eq!(request.payload_type(), PayloadType::PowerInformationRequest);
    }

    #[test]
    fn test_to_bytes() {
        let mut buffer = [0; 1024];
        let request = PowerInformationRequest {};
        assert_eq!(request.to_bytes(&mut buffer), Ok(0));
    }

    #[test]
    fn test_from_bytes_ok() {
        let mut buffer = [0; 1024];
        let bytes = PowerInformationRequest {}.to_bytes(&mut buffer).unwrap();
        let request = PowerInformationRequest::from_bytes(&buffer[..bytes]);

        assert!(
            request.is_ok(),
            "Expected PowerInformationRequest, recieved an Error."
        );
    }
}
