/// Available positive acknowledgement codes for `DiagnosticMessageAck`.
///
/// Positive acknowledgement codes from the result of a sent `DiagnosticMessage`.

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DiagnosticAckCode {
    /// Acknowledged
    Acknowledged = 0x00,
}
