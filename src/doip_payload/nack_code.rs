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
            _ => Err("Invalid NackCode."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NackCode;

    #[test]
    fn test_try_from_bytes() {
        for n in u8::MIN..=u8::MAX {
            let nack_code = NackCode::try_from(n);

            match n {
                0x00 => assert_eq!(nack_code.unwrap(), NackCode::IncorrectPatternFormat),
                0x01 => assert_eq!(nack_code.unwrap(), NackCode::UnknownPayloadType),
                0x02 => assert_eq!(nack_code.unwrap(), NackCode::MessageTooLarge),
                0x03 => assert_eq!(nack_code.unwrap(), NackCode::OutOfMemory),
                0x04 => assert_eq!(nack_code.unwrap(), NackCode::InvalidPayloadLength),
                _ => assert_eq!(nack_code.unwrap_err(), "Invalid NackCode."),
            };
        }
    }

    #[test]
    fn test_from_nack_code() {
        let u = u8::from(NackCode::IncorrectPatternFormat);
        assert_eq!(u, 0x00);
        let u = u8::from(NackCode::UnknownPayloadType);
        assert_eq!(u, 0x01);
        let u = u8::from(NackCode::MessageTooLarge);
        assert_eq!(u, 0x02);
        let u = u8::from(NackCode::OutOfMemory);
        assert_eq!(u, 0x03);
        let u = u8::from(NackCode::InvalidPayloadLength);
        assert_eq!(u, 0x04);
    }
}
