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
    /// Validates the inverse byte provided by the incoming/outgoing DoipHeader
    pub fn validate_inverse_byte(&self, inverse_byte: &u8) -> bool {
        let version_byte = self.clone() as u8;
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
            _ => Err("Invalid DoipVersion."),
        }
    }
}
