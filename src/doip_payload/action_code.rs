/// Used in Vehicle Announcement Messages to give next steps.
///
/// Used to inform the client of further actions which need to be taken on a
/// `DoIP` server.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActionCode {
    /// No Further Action Required
    NoFurtherActionRequired = 0x00,

    /// Reserved By ISO-13400 for bytes value `01`
    ReservedByIso13400_01 = 0x01,

    /// Reserved By ISO-13400 for bytes value `02`
    ReservedByIso13400_02 = 0x02,

    /// Reserved By ISO-13400 for bytes value `03`
    ReservedByIso13400_03 = 0x03,

    /// Reserved By ISO-13400 for bytes value `04`
    ReservedByIso13400_04 = 0x04,

    /// Reserved By ISO-13400 for bytes value `05`
    ReservedByIso13400_05 = 0x05,

    /// Reserved By ISO-13400 for bytes value `06`
    ReservedByIso13400_06 = 0x06,

    /// Reserved By ISO-13400 for bytes value `07`
    ReservedByIso13400_07 = 0x07,

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

    /// Routing Activation Required
    RoutingActivationRequired = 0x10,
}

impl From<ActionCode> for u8 {
    fn from(action_code: ActionCode) -> Self {
        action_code as u8
    }
}

impl TryFrom<u8> for ActionCode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(ActionCode::NoFurtherActionRequired),
            0x01 => Ok(ActionCode::ReservedByIso13400_01),
            0x02 => Ok(ActionCode::ReservedByIso13400_02),
            0x03 => Ok(ActionCode::ReservedByIso13400_03),
            0x04 => Ok(ActionCode::ReservedByIso13400_04),
            0x05 => Ok(ActionCode::ReservedByIso13400_05),
            0x06 => Ok(ActionCode::ReservedByIso13400_06),
            0x07 => Ok(ActionCode::ReservedByIso13400_07),
            0x08 => Ok(ActionCode::ReservedByIso13400_08),
            0x09 => Ok(ActionCode::ReservedByIso13400_09),
            0x0A => Ok(ActionCode::ReservedByIso13400_0A),
            0x0B => Ok(ActionCode::ReservedByIso13400_0B),
            0x0C => Ok(ActionCode::ReservedByIso13400_0C),
            0x0D => Ok(ActionCode::ReservedByIso13400_0D),
            0x0E => Ok(ActionCode::ReservedByIso13400_0E),
            0x0F => Ok(ActionCode::ReservedByIso13400_0F),
            0x10 => Ok(ActionCode::RoutingActivationRequired),
            _ => Err("Invalid ActionCode."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ActionCode;

    #[test]
    fn test_try_from_bytes() {
        for n in u8::MIN..u8::MAX {
            let action_code = ActionCode::try_from(n);

            match n {
                0x00 => assert_eq!(action_code.unwrap(), ActionCode::NoFurtherActionRequired),
                0x01 => assert_eq!(action_code.unwrap(), ActionCode::ReservedByIso13400_01),
                0x02 => assert_eq!(action_code.unwrap(), ActionCode::ReservedByIso13400_02),
                0x03 => assert_eq!(action_code.unwrap(), ActionCode::ReservedByIso13400_03),
                0x04 => assert_eq!(action_code.unwrap(), ActionCode::ReservedByIso13400_04),
                0x05 => assert_eq!(action_code.unwrap(), ActionCode::ReservedByIso13400_05),
                0x06 => assert_eq!(action_code.unwrap(), ActionCode::ReservedByIso13400_06),
                0x07 => assert_eq!(action_code.unwrap(), ActionCode::ReservedByIso13400_07),
                0x08 => assert_eq!(action_code.unwrap(), ActionCode::ReservedByIso13400_08),
                0x09 => assert_eq!(action_code.unwrap(), ActionCode::ReservedByIso13400_09),
                0x0A => assert_eq!(action_code.unwrap(), ActionCode::ReservedByIso13400_0A),
                0x0B => assert_eq!(action_code.unwrap(), ActionCode::ReservedByIso13400_0B),
                0x0C => assert_eq!(action_code.unwrap(), ActionCode::ReservedByIso13400_0C),
                0x0D => assert_eq!(action_code.unwrap(), ActionCode::ReservedByIso13400_0D),
                0x0E => assert_eq!(action_code.unwrap(), ActionCode::ReservedByIso13400_0E),
                0x0F => assert_eq!(action_code.unwrap(), ActionCode::ReservedByIso13400_0F),
                0x10 => assert_eq!(action_code.unwrap(), ActionCode::RoutingActivationRequired),
                _ => assert_eq!(action_code.unwrap_err(), "Invalid ActionCode."),
            };
        }
    }

    #[test]
    fn test_from_action_code() {
        let u = u8::from(ActionCode::NoFurtherActionRequired);
        assert_eq!(u, 0x00);
        let u = u8::from(ActionCode::ReservedByIso13400_01);
        assert_eq!(u, 0x01);
        let u = u8::from(ActionCode::ReservedByIso13400_02);
        assert_eq!(u, 0x02);
        let u = u8::from(ActionCode::ReservedByIso13400_03);
        assert_eq!(u, 0x03);
        let u = u8::from(ActionCode::ReservedByIso13400_04);
        assert_eq!(u, 0x04);
        let u = u8::from(ActionCode::ReservedByIso13400_05);
        assert_eq!(u, 0x05);
        let u = u8::from(ActionCode::ReservedByIso13400_06);
        assert_eq!(u, 0x06);
        let u = u8::from(ActionCode::ReservedByIso13400_07);
        assert_eq!(u, 0x07);
        let u = u8::from(ActionCode::ReservedByIso13400_08);
        assert_eq!(u, 0x08);
        let u = u8::from(ActionCode::ReservedByIso13400_09);
        assert_eq!(u, 0x09);
        let u = u8::from(ActionCode::ReservedByIso13400_0A);
        assert_eq!(u, 0x0A);
        let u = u8::from(ActionCode::ReservedByIso13400_0B);
        assert_eq!(u, 0x0B);
        let u = u8::from(ActionCode::ReservedByIso13400_0C);
        assert_eq!(u, 0x0C);
        let u = u8::from(ActionCode::ReservedByIso13400_0D);
        assert_eq!(u, 0x0D);
        let u = u8::from(ActionCode::ReservedByIso13400_0E);
        assert_eq!(u, 0x0E);
        let u = u8::from(ActionCode::ReservedByIso13400_0F);
        assert_eq!(u, 0x0F);
        let u = u8::from(ActionCode::RoutingActivationRequired);
        assert_eq!(u, 0x10);
    }
}
