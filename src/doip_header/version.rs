use crate::definitions::{
    DEFAULT_VALUE, ISO13400_2010, ISO13400_2012, ISO13400_2019, ISO13400_2019_AMD1, RESERVED_VER,
};

/// Avaiable version of the `DoIP` protocol as per ISO-13400.
///
/// Maps to `u8` values for avaiable `DoIP` protocols which are supported by this
/// crates and ISO-13400.
#[cfg_attr(feature = "std", pyo3::pyclass(eq, eq_int))]
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
