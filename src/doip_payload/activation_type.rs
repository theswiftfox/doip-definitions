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
