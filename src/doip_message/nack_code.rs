use std::fmt;

/// Used in `GenericNack`, `NackCode` provides the possible errors causing the
/// NACK.
///
/// Used to understand the result of a DoIP packet.
#[derive(Clone, Copy, Debug, PartialEq)]
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

impl fmt::Display for NackCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let nack_string = match self {
            NackCode::IncorrectPatternFormat => "Incorrect pattern format",
            NackCode::UnknownPayloadType => "Unknown payload type",
            NackCode::MessageTooLarge => "Message too large",
            NackCode::OutOfMemory => "Out of memory",
            NackCode::InvalidPayloadLength => "Invalid payload length",
        };
        write!(f, "{}", nack_string)
    }
}
