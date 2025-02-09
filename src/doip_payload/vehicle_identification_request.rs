use crate::header::{DoipPayload, PayloadType};

/// Used to request a `VehicleAnnouncement` from all available `DoIP` entities
/// on the network.
#[derive(Copy, Clone, Debug)]
pub struct VehicleIdentificationRequest {}

impl DoipPayload for VehicleIdentificationRequest {
    fn payload_type(&self) -> PayloadType {
        PayloadType::VehicleIdentificationRequest
    }
}
