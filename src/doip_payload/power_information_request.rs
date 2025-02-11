use crate::{header::PayloadType, DoipPayload};

/// Requests the power mode status of a `DoIP` Entity.
#[derive(Copy, Clone, Debug)]
pub struct PowerInformationRequest {}

impl DoipPayload for PowerInformationRequest {
    fn payload_type(&self) -> PayloadType {
        PayloadType::PowerInformationRequest
    }
}
