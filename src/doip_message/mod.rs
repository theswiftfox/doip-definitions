use crate::{header::DoipHeader, payload::DoipPayload};

/// The decoded struct of a `DoIP` packet.
///
/// Each `DoIP` packet contains a header which describes the message, this is outlined
/// in `DoipHeader`.
///
/// Some Payload Types available in `DoIP` require a payload which is covered by
/// `DoipPayload`.
#[derive(Debug, PartialEq, Clone)]
pub struct DoipMessage<const N: usize> {
    /// Defined by `DoipHeader`, the header supplies the information for programs
    /// to understand the payload.
    pub header: DoipHeader,

    /// Takes any struct implementing `DoipPayload`.
    pub payload: DoipPayload<N>,
}
