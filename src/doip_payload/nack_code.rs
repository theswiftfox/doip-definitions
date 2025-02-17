/// Used in `GenericNack`, `NackCode` provides the possible errors causing the
/// NACK.
///
/// Used to understand the result of a `DoIP` packet.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NackCode {
    /// Incorrect Pattern Format
    IncorrectPatternFormat = 0x00,

    /// Unknown Payload Type
    UnknownPayloadType = 0x01,

    /// Message Too Large
    MessageTooLarge = 0x02,

    /// Out Of Memory
    OutOfMemory = 0x03,

    /// Invalid Payload Length
    InvalidPayloadLength = 0x04,
}

impl From<NackCode> for u8 {
    fn from(version: NackCode) -> Self {
        version as u8
    }
}

impl TryFrom<u8> for NackCode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(NackCode::IncorrectPatternFormat),
            0x01 => Ok(NackCode::UnknownPayloadType),
            0x02 => Ok(NackCode::MessageTooLarge),
            0x03 => Ok(NackCode::OutOfMemory),
            0x04 => Ok(NackCode::InvalidPayloadLength),
            _ => Err("Invalid Nack Code."),
        }
    }
}
