use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NackCode {
    IncorrectPatternFormat = 0x00,
    UnknownPayloadType = 0x01,
    MessageTooLarge = 0x02,
    OutOfMemory = 0x03,
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