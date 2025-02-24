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

#[cfg(test)]
mod tests {
    use super::ActivationCode;

    #[test]
    fn test_try_from_bytes() {
        for n in u8::MIN..u8::MAX {
            let activation_code = ActivationCode::try_from(n);

            match n {
                0x00 => assert_eq!(
                    activation_code.unwrap(),
                    ActivationCode::DeniedUnknownSourceAddress
                ),
                0x01 => assert_eq!(
                    activation_code.unwrap(),
                    ActivationCode::DeniedTCPSocketsFull
                ),
                0x02 => assert_eq!(
                    activation_code.unwrap(),
                    ActivationCode::DeniedTCPSocketAlreadyConnected
                ),
                0x03 => assert_eq!(
                    activation_code.unwrap(),
                    ActivationCode::DeniedSourceIsAlreadyActive
                ),
                0x04 => assert_eq!(
                    activation_code.unwrap(),
                    ActivationCode::DeniedMissingAuthentication
                ),
                0x05 => assert_eq!(
                    activation_code.unwrap(),
                    ActivationCode::DeniedRejectedConfirmation
                ),
                0x06 => assert_eq!(
                    activation_code.unwrap(),
                    ActivationCode::DeniedUnsupportedRoutingActivationType
                ),
                0x07 => assert_eq!(
                    activation_code.unwrap(),
                    ActivationCode::DeniedRequestEncryptedTLSConnection
                ),
                0x08 => assert_eq!(
                    activation_code.unwrap(),
                    ActivationCode::ReservedByIso13400_08
                ),
                0x09 => assert_eq!(
                    activation_code.unwrap(),
                    ActivationCode::ReservedByIso13400_09
                ),
                0x0A => assert_eq!(
                    activation_code.unwrap(),
                    ActivationCode::ReservedByIso13400_0A
                ),
                0x0B => assert_eq!(
                    activation_code.unwrap(),
                    ActivationCode::ReservedByIso13400_0B
                ),
                0x0C => assert_eq!(
                    activation_code.unwrap(),
                    ActivationCode::ReservedByIso13400_0C
                ),
                0x0D => assert_eq!(
                    activation_code.unwrap(),
                    ActivationCode::ReservedByIso13400_0D
                ),
                0x0E => assert_eq!(
                    activation_code.unwrap(),
                    ActivationCode::ReservedByIso13400_0E
                ),
                0x0F => assert_eq!(
                    activation_code.unwrap(),
                    ActivationCode::ReservedByIso13400_0F
                ),
                0x10 => assert_eq!(
                    activation_code.unwrap(),
                    ActivationCode::SuccessfullyActivated
                ),
                0x11 => assert_eq!(
                    activation_code.unwrap(),
                    ActivationCode::ActivatedConfirmationRequired
                ),
                _ => assert_eq!(activation_code.unwrap_err(), "Invalid ActivationCode."),
            };
        }
    }

    #[test]
    fn test_fom_activation_code() {
        let u = u8::from(ActivationCode::DeniedUnknownSourceAddress);
        assert_eq!(u, 0x00);
        let u = u8::from(ActivationCode::DeniedTCPSocketsFull);
        assert_eq!(u, 0x01);
        let u = u8::from(ActivationCode::DeniedTCPSocketAlreadyConnected);
        assert_eq!(u, 0x02);
        let u = u8::from(ActivationCode::DeniedSourceIsAlreadyActive);
        assert_eq!(u, 0x03);
        let u = u8::from(ActivationCode::DeniedMissingAuthentication);
        assert_eq!(u, 0x04);
        let u = u8::from(ActivationCode::DeniedRejectedConfirmation);
        assert_eq!(u, 0x05);
        let u = u8::from(ActivationCode::DeniedUnsupportedRoutingActivationType);
        assert_eq!(u, 0x06);
        let u = u8::from(ActivationCode::DeniedRequestEncryptedTLSConnection);
        assert_eq!(u, 0x07);
        let u = u8::from(ActivationCode::ReservedByIso13400_08);
        assert_eq!(u, 0x08);
        let u = u8::from(ActivationCode::ReservedByIso13400_09);
        assert_eq!(u, 0x09);
        let u = u8::from(ActivationCode::ReservedByIso13400_0A);
        assert_eq!(u, 0x0A);
        let u = u8::from(ActivationCode::ReservedByIso13400_0B);
        assert_eq!(u, 0x0B);
        let u = u8::from(ActivationCode::ReservedByIso13400_0C);
        assert_eq!(u, 0x0C);
        let u = u8::from(ActivationCode::ReservedByIso13400_0D);
        assert_eq!(u, 0x0D);
        let u = u8::from(ActivationCode::ReservedByIso13400_0E);
        assert_eq!(u, 0x0E);
        let u = u8::from(ActivationCode::ReservedByIso13400_0F);
        assert_eq!(u, 0x0F);
        let u = u8::from(ActivationCode::SuccessfullyActivated);
        assert_eq!(u, 0x10);
        let u = u8::from(ActivationCode::ActivatedConfirmationRequired);
        assert_eq!(u, 0x11);
    }
}
