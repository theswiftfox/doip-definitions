use crate::{
    error::PayloadError,
    header::{DoipPayload, PayloadType},
};

/// Used to request a `VehicleAnnouncement` from all available DoIP entities
/// on the network.
#[derive(Copy, Clone, Debug)]
pub struct VehicleIdentificationRequest {}

impl DoipPayload<'_> for VehicleIdentificationRequest {
    fn payload_type(&self) -> PayloadType {
        PayloadType::VehicleIdentificationRequest
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
        doip_message::vehicle_identification_request::VehicleIdentificationRequest,
        header::{DoipPayload, PayloadType},
    };

    #[test]
    fn test_payload_type() {
        let request = VehicleIdentificationRequest {};
        assert_eq!(
            request.payload_type(),
            PayloadType::VehicleIdentificationRequest
        );
    }

    #[test]
    fn test_to_bytes() {
        let mut buffer = [0; 1024];
        let request = VehicleIdentificationRequest {};
        assert_eq!(request.to_bytes(&mut buffer), Ok(0));
    }

    #[test]
    fn test_from_bytes_ok() {
        let mut buffer = [0; 1024];
        let bytes = VehicleIdentificationRequest {}
            .to_bytes(&mut buffer)
            .unwrap();
        let request = VehicleIdentificationRequest::from_bytes(&buffer[..bytes]);

        assert!(
            request.is_ok(),
            "Expected VehicleIdentificationRequest, received an Error."
        );
    }
}
