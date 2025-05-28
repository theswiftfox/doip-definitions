use derive_more::From;

/// Custom Result type which allows for easier typing of errors across the API.
pub type Result<T> = core::result::Result<T, Error>;

/// Custom Error enum for deriving error types across the application and API.
///
/// Expand to fit new dependencies using `#[from]` and implement custom error types
/// with context.
#[derive(Debug, From)]
pub enum Error {
    OutOfBounds {
        source: &'static str,
        variable: &'static str,
    },
    SliceTooSmall,
    InvalidProtocolVersion {
        value: u8,
    },
    InvalidNackCode {
        value: u8,
    },
    InvalidActionCode {
        value: u8,
    },
    InvalidSyncStatus {
        value: u8,
    },
    InvalidActivationCode {
        value: u8,
    },
    InvalidActivationType {
        value: u8,
    },
    InvalidNodeType {
        value: u8,
    },
    InvalidPowerMode {
        value: u8,
    },
    InvalidDiagnosticNackCode {
        value: u8,
    },
    InvalidDiagnosticAckCode {
        value: u8,
    },
    InvalidPayloadType {
        value: [u8; 2],
    },
    BufferTooSmall {
        size: usize,
    },

    /// Derived implementation for standard library IO errors
    #[from]
    #[allow(clippy::enum_variant_names)]
    SliceError(core::array::TryFromSliceError),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
