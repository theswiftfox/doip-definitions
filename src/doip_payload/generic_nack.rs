use crate::payload::NackCode;

/// The generic negative acknowledgement of a bad request.
///
/// This is found usually when a critical error occurs due to a bad `DoIP` packet
/// or an entity issue.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GenericNack {
    /// Available negative acknowledgement codes
    pub nack_code: NackCode,
}
