use crate::{header::PayloadType, DoipPayload};

/// Requests the status of a `DoIP` Entity.
#[derive(Copy, Clone, Debug)]
pub struct EntityStatusRequest {}

impl DoipPayload for EntityStatusRequest {
    fn payload_type(&self) -> PayloadType {
        PayloadType::EntityStatusRequest
    }
}
