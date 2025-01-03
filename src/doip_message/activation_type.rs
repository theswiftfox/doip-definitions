use std::fmt;

/// Used in Routing Activation Request to request specific routing types.
///
/// Used to customise the routing type requested from the DoIP entity for different
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

impl fmt::Display for ActivationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let activation_string = match self {
            ActivationType::Default => "Default",
            ActivationType::WwhObd => "WWH-OBD",
            ActivationType::CentralSecurity => "Central security",
        };
        write!(f, "{}", activation_string)
    }
}
