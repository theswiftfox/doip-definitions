/// Available negative acknowledgement codes for `DiagnosticMessageAck`.
///
/// Negative acknowledgement codes from the result of a sent `DiagnosticMessage`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DiagnosticNackCode {
    /// Reserved By ISO-13400 for bytes value `00`
    ReservedByIso13400_00 = 0x00,

    /// Reserved By ISO-13400 for bytes value `01`
    ReservedByIso13400_01 = 0x01,

    /// Invalid Source Address
    InvalidSourceAddress = 0x02,

    /// Unknown Target Address
    UnknownTargetAddress = 0x03,

    /// Diagnostic Message Too Large
    DiagnosticMessageTooLarge = 0x04,

    /// Out Of Memory
    OutOfMemory = 0x05,

    /// Target Unreachable
    TargetUnreachable = 0x06,

    /// Unknown Network
    UnknownNetwork = 0x07,

    /// Transport Protocol Error
    TransportProtocolError = 0x08,
}

impl From<DiagnosticNackCode> for u8 {
    fn from(diagnostic_nack_code: DiagnosticNackCode) -> Self {
        diagnostic_nack_code as u8
    }
}

impl TryFrom<u8> for DiagnosticNackCode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(DiagnosticNackCode::ReservedByIso13400_00),
            0x01 => Ok(DiagnosticNackCode::ReservedByIso13400_01),
            0x02 => Ok(DiagnosticNackCode::InvalidSourceAddress),
            0x03 => Ok(DiagnosticNackCode::UnknownTargetAddress),
            0x04 => Ok(DiagnosticNackCode::DiagnosticMessageTooLarge),
            0x05 => Ok(DiagnosticNackCode::OutOfMemory),
            0x06 => Ok(DiagnosticNackCode::TargetUnreachable),
            0x07 => Ok(DiagnosticNackCode::UnknownNetwork),
            0x08 => Ok(DiagnosticNackCode::TransportProtocolError),
            _ => Err("Invalid DiagnosticNackCode."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DiagnosticNackCode;

    #[test]
    fn test_exhaustive() {
        for n in u8::MIN..u8::MAX {
            let negative_ack_code = DiagnosticNackCode::try_from(n);

            match n {
                0x00 => assert_eq!(
                    negative_ack_code.unwrap(),
                    DiagnosticNackCode::ReservedByIso13400_00
                ),
                0x01 => assert_eq!(
                    negative_ack_code.unwrap(),
                    DiagnosticNackCode::ReservedByIso13400_01
                ),
                0x02 => assert_eq!(
                    negative_ack_code.unwrap(),
                    DiagnosticNackCode::InvalidSourceAddress
                ),
                0x03 => assert_eq!(
                    negative_ack_code.unwrap(),
                    DiagnosticNackCode::UnknownTargetAddress
                ),
                0x04 => assert_eq!(
                    negative_ack_code.unwrap(),
                    DiagnosticNackCode::DiagnosticMessageTooLarge
                ),
                0x05 => assert_eq!(negative_ack_code.unwrap(), DiagnosticNackCode::OutOfMemory),
                0x06 => assert_eq!(
                    negative_ack_code.unwrap(),
                    DiagnosticNackCode::TargetUnreachable
                ),
                0x07 => assert_eq!(
                    negative_ack_code.unwrap(),
                    DiagnosticNackCode::UnknownNetwork
                ),
                0x08 => assert_eq!(
                    negative_ack_code.unwrap(),
                    DiagnosticNackCode::TransportProtocolError
                ),
                _ => assert_eq!(
                    negative_ack_code.unwrap_err(),
                    "Invalid DiagnosticNackCode."
                ),
            };
        }
    }
}
