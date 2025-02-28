/// Available positive acknowledgement codes for `DiagnosticMessageAck`.
///
/// Positive acknowledgement codes from the result of a sent `DiagnosticMessage`.
#[cfg_attr(feature = "std", pyo3::pyclass(eq, eq_int))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DiagnosticAckCode {
    /// Acknowledged
    Acknowledged = 0x00,
}
