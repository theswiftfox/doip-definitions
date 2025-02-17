/// Used in `PowerInformationResponse`, `PowerMode` provides the power mode that
/// the `DoIP` entity can be.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PowerMode {
    /// Not Ready
    NotReady = 0x00,

    /// Ready
    Ready = 0x01,

    /// Not Supported
    NotSupported = 0x02,
}

impl From<PowerMode> for u8 {
    fn from(power_mode: PowerMode) -> Self {
        power_mode as u8
    }
}

impl TryFrom<u8> for PowerMode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(PowerMode::NotReady),
            0x01 => Ok(PowerMode::Ready),
            0x02 => Ok(PowerMode::NotSupported),
            _ => Err("Invalid Power Mode."),
        }
    }
}
