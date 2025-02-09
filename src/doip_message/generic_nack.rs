use crate::{
    header::{DoipPayload, PayloadType},
    message::NackCode,
};

/// The generic negative acknowledgement of a bad request.
///
/// This is found usually when a critical error occurs due to a bad `DoIP` packet
/// or an entity issue.
#[derive(Copy, Clone, Debug)]
pub struct GenericNack {
    /// Available negative acknowledgement codes
    pub nack_code: NackCode,
}

impl DoipPayload for GenericNack {
    fn payload_type(&self) -> PayloadType {
        PayloadType::GenericNack
    }
}
