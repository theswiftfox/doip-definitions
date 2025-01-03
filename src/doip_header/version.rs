use std::fmt;

use crate::definitions::{
    DEFAULT_VALUE, ISO13400_2010, ISO13400_2012, ISO13400_2019, ISO13400_2019_AMD1, RESERVED_VER,
};

/// Avaiable version of the DoIP protocol as per ISO-13400.
///
/// Maps to `u8` values for avaiable DoIP protocols which are supported by this
/// crates and ISO-13400.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum DoipVersion {
    /// Reserved Version
    ReservedVer = RESERVED_VER,

    /// DoIP Payload Version: ISO-13400 2010 Version
    Iso13400_2010 = ISO13400_2010,

    /// DoIP Payload Version: ISO-13400 2012 Version
    Iso13400_2012 = ISO13400_2012,

    /// DoIP Payload Version: ISO-13400 2019 Version
    Iso13400_2019 = ISO13400_2019,

    /// DoIP Payload Version: ISO-13400 2019_AMD1 Version
    Iso13400_2019Amd1 = ISO13400_2019_AMD1,

    /// DoIP Payload Version: Default Version
    DefaultValue = DEFAULT_VALUE,
}

impl fmt::Display for DoipVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let version_string = match self {
            DoipVersion::ReservedVer => "Reserved",
            DoipVersion::Iso13400_2010 => "DoIP ISO/DIS 13400-2:2010",
            DoipVersion::Iso13400_2012 => "DoIP ISO 13400-2:2012",
            DoipVersion::Iso13400_2019 => "DoIP ISO 13400-2:2019",
            DoipVersion::Iso13400_2019Amd1 => "DoIP ISO 13400-2:2019 Amd1 (experimental)",
            DoipVersion::DefaultValue => {
                "Default value for vehicle identification request messages"
            }
        };
        write!(f, "{}", version_string)
    }
}

impl DoipVersion {
    /// Convert DoipVersion value to u8.
    pub fn to_u8(self) -> u8 {
        self as u8
    }

    /// Convert DoipVersion value from u8.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            RESERVED_VER => Some(DoipVersion::ReservedVer),
            ISO13400_2010 => Some(DoipVersion::Iso13400_2010),
            ISO13400_2012 => Some(DoipVersion::Iso13400_2012),
            ISO13400_2019 => Some(DoipVersion::Iso13400_2019),
            ISO13400_2019_AMD1 => Some(DoipVersion::Iso13400_2019Amd1),
            DEFAULT_VALUE => Some(DoipVersion::DefaultValue),
            _ => None,
        }
    }
}
