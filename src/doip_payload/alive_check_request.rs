use crate::header::{DoipPayload, PayloadType};

/// Checks the TCP Socket is still alive
///
/// Sent with no payload, the `AliveCheckRequest` is utilised to maintain a connection
/// to a TCP socket or to check the status of one.
#[derive(Copy, Clone, Debug)]
pub struct AliveCheckRequest {}

impl DoipPayload for AliveCheckRequest {
    fn payload_type(&self) -> PayloadType {
        PayloadType::AliveCheckRequest
    }
}
