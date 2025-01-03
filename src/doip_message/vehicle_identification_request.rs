use crate::{
    error::PayloadError,
    header::{DoipPayload, PayloadType},
};

/// Used to request a `VehicleAnnouncement` from all available DoIP entities
/// on the network.
#[derive(Copy, Clone, Debug)]
pub struct VehicleIdentificationRequest {}

impl DoipPayload for VehicleIdentificationRequest {
    fn payload_type(&self) -> PayloadType {
        PayloadType::VehicleIdentificationRequest
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![]
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
        let request = VehicleIdentificationRequest {};
        assert_eq!(request.to_bytes(), vec![]);
    }

    #[test]
    fn test_from_bytes_ok() {
        let bytes = VehicleIdentificationRequest {}.to_bytes();
        let request = VehicleIdentificationRequest::from_bytes(&bytes);

        assert!(
            request.is_ok(),
            "Expected VehicleIdentificationRequest, received an Error."
        );
    }
}
