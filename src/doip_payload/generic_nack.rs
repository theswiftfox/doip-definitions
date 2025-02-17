use crate::payload::NackCode;

/// The generic negative acknowledgement of a bad request.
///
/// This is found usually when a critical error occurs due to a bad `DoIP` packet
/// or an entity issue.
#[derive(Copy, Clone, Debug)]
pub struct GenericNack {
    /// Available negative acknowledgement codes
    pub nack_code: NackCode,
}

impl From<GenericNack> for [u8; 1] {
    fn from(generic_nack: GenericNack) -> Self {
        [generic_nack.nack_code as u8]
    }
}

impl TryFrom<[u8; 1]> for GenericNack {
    type Error = &'static str;

    fn try_from(value: [u8; 1]) -> Result<Self, Self::Error> {
        let nack_code = NackCode::try_from(value[0])?;
        Ok(GenericNack { nack_code })
    }
}
