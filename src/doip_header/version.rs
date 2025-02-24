use crate::definitions::{
    DEFAULT_VALUE, ISO13400_2010, ISO13400_2012, ISO13400_2019, ISO13400_2019_AMD1, RESERVED_VER,
};

/// Avaiable version of the `DoIP` protocol as per ISO-13400.
///
/// Maps to `u8` values for avaiable `DoIP` protocols which are supported by this
/// crates and ISO-13400.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ProtocolVersion {
    /// Reserved Version
    ReservedVer = RESERVED_VER,

    /// `DoIP` Payload Version: ISO-13400 2010 Version
    Iso13400_2010 = ISO13400_2010,

    /// `DoIP` Payload Version: ISO-13400 2012 Version
    Iso13400_2012 = ISO13400_2012,

    /// `DoIP` Payload Version: ISO-13400 2019 Version
    Iso13400_2019 = ISO13400_2019,

    /// `DoIP` Payload Version: ISO-13400 `2019_AMD1` Version
    Iso13400_2019Amd1 = ISO13400_2019_AMD1,

    /// `DoIP` Payload Version: Default Version
    DefaultValue = DEFAULT_VALUE,
}

impl ProtocolVersion {
    /// Validates the inverse byte provided by the incoming/outgoing `DoipHeader`
    #[must_use]
    pub fn validate_inverse_byte(&self, inverse_byte: &u8) -> bool {
        let version_byte = *self as u8;
        !inverse_byte == version_byte
    }
}

impl From<ProtocolVersion> for u8 {
    fn from(version: ProtocolVersion) -> Self {
        version as u8
    }
}

impl TryFrom<u8> for ProtocolVersion {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            RESERVED_VER => Ok(ProtocolVersion::ReservedVer),
            ISO13400_2010 => Ok(ProtocolVersion::Iso13400_2010),
            ISO13400_2012 => Ok(ProtocolVersion::Iso13400_2012),
            ISO13400_2019 => Ok(ProtocolVersion::Iso13400_2019),
            ISO13400_2019_AMD1 => Ok(ProtocolVersion::Iso13400_2019Amd1),
            DEFAULT_VALUE => Ok(ProtocolVersion::DefaultValue),
            _ => Err("Invalid ProtocolVersion."),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::definitions::{
        DEFAULT_VALUE, ISO13400_2010, ISO13400_2012, ISO13400_2019, ISO13400_2019_AMD1,
        RESERVED_VER,
    };

    use super::ProtocolVersion;

    #[test]
    fn test_try_from_bytes() {
        for n in u8::MIN..=u8::MAX {
            let protocol_version = ProtocolVersion::try_from(n);

            match n {
                RESERVED_VER => assert_eq!(protocol_version.unwrap(), ProtocolVersion::ReservedVer),
                ISO13400_2010 => {
                    assert_eq!(protocol_version.unwrap(), ProtocolVersion::Iso13400_2010)
                }
                ISO13400_2012 => {
                    assert_eq!(protocol_version.unwrap(), ProtocolVersion::Iso13400_2012)
                }
                ISO13400_2019 => {
                    assert_eq!(protocol_version.unwrap(), ProtocolVersion::Iso13400_2019)
                }
                ISO13400_2019_AMD1 => {
                    assert_eq!(
                        protocol_version.unwrap(),
                        ProtocolVersion::Iso13400_2019Amd1
                    )
                }
                DEFAULT_VALUE => {
                    assert_eq!(protocol_version.unwrap(), ProtocolVersion::DefaultValue)
                }
                _ => assert_eq!(protocol_version.unwrap_err(), "Invalid ProtocolVersion."),
            };
        }
    }

    #[test]
    fn test_from_version() {
        let u = u8::from(ProtocolVersion::DefaultValue);
        assert_eq!(u, 0xff);
        let u = u8::from(ProtocolVersion::Iso13400_2010);
        assert_eq!(u, 0x01);
        let u = u8::from(ProtocolVersion::Iso13400_2012);
        assert_eq!(u, 0x02);
        let u = u8::from(ProtocolVersion::Iso13400_2019);
        assert_eq!(u, 0x03);
        let u = u8::from(ProtocolVersion::Iso13400_2019Amd1);
        assert_eq!(u, 0x04);
        let u = u8::from(ProtocolVersion::ReservedVer);
        assert_eq!(u, 0x00);
    }

    #[test]
    fn test_validate_inverse_byte() {
        let u = ProtocolVersion::DefaultValue;

        for n in u8::MIN..=u8::MAX {
            let v = u.validate_inverse_byte(&n);

            match n {
                0x00 => assert_eq!(v, true),
                _ => assert_eq!(v, false),
            }
        }

        let u = ProtocolVersion::Iso13400_2010;

        for n in u8::MIN..=u8::MAX {
            let v = u.validate_inverse_byte(&n);

            match n {
                0xfe => assert_eq!(v, true),
                _ => assert_eq!(v, false),
            }
        }

        let u = ProtocolVersion::Iso13400_2012;

        for n in u8::MIN..=u8::MAX {
            let v = u.validate_inverse_byte(&n);

            match n {
                0xfd => assert_eq!(v, true),
                _ => assert_eq!(v, false),
            }
        }

        let u = ProtocolVersion::Iso13400_2019;

        for n in u8::MIN..=u8::MAX {
            let v = u.validate_inverse_byte(&n);

            match n {
                0xfc => assert_eq!(v, true),
                _ => assert_eq!(v, false),
            }
        }

        let u = ProtocolVersion::Iso13400_2019Amd1;

        for n in u8::MIN..=u8::MAX {
            let v = u.validate_inverse_byte(&n);

            match n {
                0xfb => assert_eq!(v, true),
                _ => assert_eq!(v, false),
            }
        }

        let u = ProtocolVersion::ReservedVer;

        for n in u8::MIN..=u8::MAX {
            let v = u.validate_inverse_byte(&n);

            match n {
                0xff => assert_eq!(v, true),
                _ => assert_eq!(v, false),
            }
        }
    }
}
