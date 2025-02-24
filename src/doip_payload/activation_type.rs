/// Used in Routing Activation Request to request specific routing types.
///
/// Used to customise the routing type requested from the `DoIP` entity for different
/// scenarios.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActivationType {
    /// Default
    Default = 0x00,

    /// WWH-OBD
    WwhObd = 0x01,

    /// Central Security
    CentralSecurity = 0x02,
}

impl From<ActivationType> for u8 {
    fn from(activation_type: ActivationType) -> Self {
        activation_type as u8
    }
}

impl TryFrom<u8> for ActivationType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ActivationType::Default),
            0x01 => Ok(ActivationType::WwhObd),
            0x02 => Ok(ActivationType::CentralSecurity),
            _ => Err("Invalid ActivationType."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ActivationType;

    #[test]
    fn test_try_from_bytes() {
        for n in u8::MIN..u8::MAX {
            let activation_type = ActivationType::try_from(n);

            match n {
                0x00 => assert_eq!(activation_type.unwrap(), ActivationType::Default),
                0x01 => assert_eq!(activation_type.unwrap(), ActivationType::WwhObd),
                0x02 => assert_eq!(activation_type.unwrap(), ActivationType::CentralSecurity),
                _ => assert_eq!(activation_type.unwrap_err(), "Invalid ActivationType."),
            };
        }
    }

    #[test]
    fn test_from_acivation_type() {
        let u = u8::from(ActivationType::Default);
        assert_eq!(u, 0x00);
        let u = u8::from(ActivationType::WwhObd);
        assert_eq!(u, 0x01);
        let u = u8::from(ActivationType::CentralSecurity);
        assert_eq!(u, 0x02);
    }
}
