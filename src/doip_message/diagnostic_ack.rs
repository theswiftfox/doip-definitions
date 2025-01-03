use std::fmt;

/// Available positive acknowledgement codes for `DiagnosticMessageAck`.
///
/// Positive acknowledgement codes from the result of a sent `DiagnosticMessage`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DiagnosticAckCode {
    /// Acknowledged
    Acknowledged = 0x00,
}

impl fmt::Display for DiagnosticAckCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let diag_strings = match self {
            DiagnosticAckCode::Acknowledged => "Acknowledged",
        };
        write!(f, "{}", diag_strings)
    }
}
