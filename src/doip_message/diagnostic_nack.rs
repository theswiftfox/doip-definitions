use std::fmt;

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

impl fmt::Display for DiagnosticNackCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let diag_strings = match self {
            DiagnosticNackCode::ReservedByIso13400_00 => "Reserved by ISO 13400",
            DiagnosticNackCode::ReservedByIso13400_01 => "Reserved by ISO 13400",
            DiagnosticNackCode::InvalidSourceAddress => "Invalid source address",
            DiagnosticNackCode::UnknownTargetAddress => "Unknown target address",
            DiagnosticNackCode::DiagnosticMessageTooLarge => "Diagnostic message too large",
            DiagnosticNackCode::OutOfMemory => "Out of memory",
            DiagnosticNackCode::TargetUnreachable => "Target unreachable",
            DiagnosticNackCode::UnknownNetwork => "Unknown network",
            DiagnosticNackCode::TransportProtocolError => "Transport protocol error",
        };
        write!(f, "{}", diag_strings)
    }
}
