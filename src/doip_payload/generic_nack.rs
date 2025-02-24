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

#[cfg(test)]
mod tests {
    use crate::payload::NackCode;

    use super::GenericNack;

    #[test]
    fn test_try_from_bytes() {
        for a in u8::MIN..u8::MAX {
            let gen_nack = GenericNack::try_from([a]);

            match a {
                0x00 => assert_eq!(
                    gen_nack.unwrap(),
                    GenericNack {
                        nack_code: NackCode::IncorrectPatternFormat
                    }
                ),
                0x01 => assert_eq!(
                    gen_nack.unwrap(),
                    GenericNack {
                        nack_code: NackCode::UnknownPayloadType
                    }
                ),
                0x02 => assert_eq!(
                    gen_nack.unwrap(),
                    GenericNack {
                        nack_code: NackCode::MessageTooLarge
                    }
                ),
                0x03 => assert_eq!(
                    gen_nack.unwrap(),
                    GenericNack {
                        nack_code: NackCode::OutOfMemory
                    }
                ),
                0x04 => assert_eq!(
                    gen_nack.unwrap(),
                    GenericNack {
                        nack_code: NackCode::InvalidPayloadLength
                    }
                ),
                _ => assert_eq!(gen_nack.unwrap_err(), "Invalid NackCode."),
            };
        }
    }

    #[test]
    fn test_from_generic_nack() {
        let gen_nack = GenericNack {
            nack_code: NackCode::IncorrectPatternFormat,
        };
        let gen_nack_bytes = <[u8; 1]>::from(gen_nack);
        assert_eq!(gen_nack_bytes, [0x00])
    }
}
