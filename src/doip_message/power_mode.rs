use std::fmt;

/// Used in `PowerInformationResponse`, `PowerMode` provides the power mode that
/// the DoIP entity can be.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PowerMode {
    /// Not Ready
    NotReady = 0x00,

    /// Ready
    Ready = 0x01,

    /// Not Supported
    NotSupported = 0x02,
}

impl fmt::Display for PowerMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let power_strings = match self {
            PowerMode::NotReady => "Not ready",
            PowerMode::Ready => "Ready",
            PowerMode::NotSupported => "Not supported",
        };
        write!(f, "{}", power_strings)
    }
}
