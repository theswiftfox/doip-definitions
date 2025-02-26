use crate::{definitions::DOIP_GENERIC_NACK_LEN, payload::NackCode};

/// The generic negative acknowledgement of a bad request.
///
/// This is found usually when a critical error occurs due to a bad `DoIP` packet
/// or an entity issue.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GenericNack {
    /// Available negative acknowledgement codes
    pub nack_code: NackCode,
}

impl<const N: usize> TryFrom<GenericNack> for [u8; N] {
    type Error = &'static str;

    fn try_from(generic_nack: GenericNack) -> Result<Self, Self::Error> {
        DOIP_GENERIC_NACK_LEN
            .le(&N)
            .then_some(())
            .ok_or("Buffer is too small")?;

        let mut buffer = [0; N];
        let offset = 0;

        buffer[offset] = generic_nack.nack_code.into();

        Ok(buffer)
    }
}

impl TryFrom<[u8; 1]> for GenericNack {
    type Error = &'static str;

    fn try_from(value: [u8; 1]) -> Result<Self, Self::Error> {
        Ok(GenericNack {
            nack_code: NackCode::try_from(value[0])?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::payload::NackCode;

    use super::GenericNack;

    #[test]
    fn test_try_from_bytes() {
        for a in u8::MIN..=u8::MAX {
            let gen_nack = GenericNack::try_from([a]);

            match a {
                0x00 => assert_eq!(
                    gen_nack,
                    Ok(GenericNack {
                        nack_code: NackCode::IncorrectPatternFormat
                    })
                ),
                0x01 => assert_eq!(
                    gen_nack,
                    Ok(GenericNack {
                        nack_code: NackCode::UnknownPayloadType
                    })
                ),
                0x02 => assert_eq!(
                    gen_nack,
                    Ok(GenericNack {
                        nack_code: NackCode::MessageTooLarge
                    })
                ),
                0x03 => assert_eq!(
                    gen_nack,
                    Ok(GenericNack {
                        nack_code: NackCode::OutOfMemory
                    })
                ),
                0x04 => assert_eq!(
                    gen_nack,
                    Ok(GenericNack {
                        nack_code: NackCode::InvalidPayloadLength
                    })
                ),
                _ => assert_eq!(gen_nack, Err("Invalid NackCode.")),
            }
        }
    }

    #[test]
    fn test_from_generic_nack_success() {
        let gen_nack = GenericNack {
            nack_code: NackCode::IncorrectPatternFormat,
        };
        let gen_nack_bytes = <[u8; 1]>::try_from(gen_nack);
        assert_eq!(gen_nack_bytes, Ok([0x00]))
    }
}
