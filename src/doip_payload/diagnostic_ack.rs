/// Available positive acknowledgement codes for `DiagnosticMessageAck`.
///
/// Positive acknowledgement codes from the result of a sent `DiagnosticMessage`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DiagnosticAckCode {
    /// Acknowledged
    Acknowledged = 0x00,
}

impl From<DiagnosticAckCode> for u8 {
    fn from(diagnostic_ack_code: DiagnosticAckCode) -> Self {
        diagnostic_ack_code as u8
    }
}

impl TryFrom<u8> for DiagnosticAckCode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(DiagnosticAckCode::Acknowledged),
            _ => Err("Invalid DiagnosticAckCode."),
        }
    }
}
