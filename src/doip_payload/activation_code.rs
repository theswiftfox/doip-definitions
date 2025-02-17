/// Used in Routing Activation Response for results from a Routing Activation
/// Request.
///
/// Used to understand the result of a Routing Activation Request to understand
/// which logical route to take.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActivationCode {
    /// Denied Unknown Source Address
    DeniedUnknownSourceAddress = 0x00,

    /// Denied TCP Sockets Full
    DeniedTCPSocketsFull = 0x01,

    /// Denied TCP Socket Already Connected
    DeniedTCPSocketAlreadyConnected = 0x02,

    /// Denied Source Is Already Active
    DeniedSourceIsAlreadyActive = 0x03,

    /// Denied Missing Authentication
    DeniedMissingAuthentication = 0x04,

    /// Denied Rejected Confirmation
    DeniedRejectedConfirmation = 0x05,

    /// Denied Unsupported Routing `ActivationType`
    DeniedUnsupportedRoutingActivationType = 0x06,

    /// Denied Request Encrypted TLS Connection
    DeniedRequestEncryptedTLSConnection = 0x07,

    /// Reserved By ISO-13400 for bytes value `08`
    ReservedByIso13400_08 = 0x08,

    /// Reserved By ISO-13400 for bytes value `09`
    ReservedByIso13400_09 = 0x09,

    /// Reserved By ISO-13400 for bytes value `0A`
    ReservedByIso13400_0A = 0x0A,

    /// Reserved By ISO-13400 for bytes value `0B`
    ReservedByIso13400_0B = 0x0B,

    /// Reserved By ISO-13400 for bytes value `0C`
    ReservedByIso13400_0C = 0x0C,

    /// Reserved By ISO-13400 for bytes value `0D`
    ReservedByIso13400_0D = 0x0D,

    /// Reserved By ISO-13400 for bytes value `0E`
    ReservedByIso13400_0E = 0x0E,

    /// Reserved By ISO-13400 for bytes value `0F`
    ReservedByIso13400_0F = 0x0F,

    /// Successfully Activated
    SuccessfullyActivated = 0x10,

    /// Activated Confirmation Required
    ActivatedConfirmationRequired = 0x11,
}

impl From<ActivationCode> for u8 {
    fn from(activation_code: ActivationCode) -> Self {
        activation_code as u8
    }
}

impl TryFrom<u8> for ActivationCode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ActivationCode::DeniedUnknownSourceAddress),
            0x01 => Ok(ActivationCode::DeniedTCPSocketsFull),
            0x02 => Ok(ActivationCode::DeniedTCPSocketAlreadyConnected),
            0x03 => Ok(ActivationCode::DeniedSourceIsAlreadyActive),
            0x04 => Ok(ActivationCode::DeniedMissingAuthentication),
            0x05 => Ok(ActivationCode::DeniedRejectedConfirmation),
            0x06 => Ok(ActivationCode::DeniedUnsupportedRoutingActivationType),
            0x07 => Ok(ActivationCode::DeniedRequestEncryptedTLSConnection),
            0x08 => Ok(ActivationCode::ReservedByIso13400_08),
            0x09 => Ok(ActivationCode::ReservedByIso13400_09),
            0x0A => Ok(ActivationCode::ReservedByIso13400_0A),
            0x0B => Ok(ActivationCode::ReservedByIso13400_0B),
            0x0C => Ok(ActivationCode::ReservedByIso13400_0C),
            0x0D => Ok(ActivationCode::ReservedByIso13400_0D),
            0x0E => Ok(ActivationCode::ReservedByIso13400_0E),
            0x0F => Ok(ActivationCode::ReservedByIso13400_0F),
            0x10 => Ok(ActivationCode::SuccessfullyActivated),
            0x11 => Ok(ActivationCode::ActivatedConfirmationRequired),
            _ => Err("Invalid ActivationCode."),
        }
    }
}
