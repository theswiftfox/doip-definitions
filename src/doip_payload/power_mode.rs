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
            _ => Err("Invalid PowerMode."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PowerMode;

    #[test]
    fn test_try_from_bytes() {
        for n in u8::MIN..=u8::MAX {
            let power_mode = PowerMode::try_from(n);

            match n {
                0x00 => assert_eq!(power_mode.unwrap(), PowerMode::NotReady),
                0x01 => assert_eq!(power_mode.unwrap(), PowerMode::Ready),
                0x02 => assert_eq!(power_mode.unwrap(), PowerMode::NotSupported),
                _ => assert_eq!(power_mode.unwrap_err(), "Invalid PowerMode."),
            };
        }
    }

    #[test]
    fn test_from_nack_code() {
        let u = u8::from(PowerMode::NotReady);
        assert_eq!(u, 0x00);
        let u = u8::from(PowerMode::Ready);
        assert_eq!(u, 0x01);
        let u = u8::from(PowerMode::NotSupported);
        assert_eq!(u, 0x02);
    }
}
