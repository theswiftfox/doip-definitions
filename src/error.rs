use thiserror::Error;

/// The root of all error types within this crate.
///
/// Built purely for descriptive errors during tracebacks and development
/// the `ParseError` houses the error types down to the individual possible errors
/// each `PayloadType` can come across.
#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    /// When an empty buffer is passed into a `DoipMessage::parse_from_bytes`.
    #[error("empty input")]
    EmptyInput,

    /// When a buffer contains an invalid protocol response from the server.
    #[error("invalid protocol version")]
    InvalidProtocolVersion,

    /// When the Inverse Protocol Version is not the inverse byte of the protocol
    /// version.
    #[error("failed protocol check")]
    FailedProtocolCheck,

    /// When the `DoipMessage::parse_from_bytes` attemps to get a byte in which
    /// there is no index available.
    #[error("index failure")]
    IndexFailure,

    /// When the length of the buffer minus the header size is less than the
    /// `Payload Length` in the header.
    #[error("incomplete payload")]
    IncompletePayload,

    /// Error parsing a specific payload.
    #[error("payload parse error")]
    PayloadParse(#[from] PayloadError),
}

/// Specific payload type errors.
///
/// Most of these errors are derived from the failure paths of parsing the buffer
/// into a `DoipMessage`
#[derive(Error, Debug, PartialEq)]
pub enum PayloadError {
    /// Parse error for Alive Check Response payload type.
    #[error("alive check payload parse failure")]
    AliveCheckResponseParseError(#[from] AliveCheckResponseError),

    /// Parse error for Diagnostic Message Nack payload type.
    #[error("diuagnostic message negative acknolwedgement payload parse failure")]
    DiagnosticMessageNackParseError(#[from] DiagnosticMessageNackError),

    /// Parse error for Diagnostic Message Ack payload type.
    #[error("diagnostic message acknowledgement payload parse failure")]
    DiagnosticMessageAckError(#[from] DiagnosticMessageAckError),

    /// Parse error for Diagnostic Message payload type.
    #[error("diagnostic message payload parse failure")]
    DiagnosticMessageError(#[from] DiagnosticMessageError),

    /// Parse error for Entity Status Response payload type.
    #[error("entity status response payload parse failure")]
    EntityStatusResponseError(#[from] EntityStatusResponseError),

    /// Parse error for Generic Nack payload type.
    #[error("generic nack payload parse failure")]
    GenericNackError(#[from] GenericNackError),

    /// Parse error for Power Information Response payload type.
    #[error("power information response payload parse failure")]
    PowerInformationResponseError(#[from] PowerInformationResponseError),

    /// Parse error for Routing Activation Request payload type.
    #[error("routing activation request payload parse failure")]
    RoutingActivationRequestError(#[from] RoutingActivationRequestError),

    /// Parse error for Routing Activation Response payload type.
    #[error("routing activation response payload parse failure")]
    RoutingActivationResponseError(#[from] RoutingActivationResponseError),

    /// Parse error for Vehicle Announcement Message payload type.
    #[error("vehicle announcement message payload parse failure")]
    VehicleAnnouncementMessageError(#[from] VehicleAnnouncementMessageError),

    /// Parse error for Vehicle Identification Request Eid payload type.
    #[error("vehicle identification request with eid payload parse failure")]
    VehicleIdentificationRequestEidError(#[from] VehicleIdentificationRequestEidError),

    /// Parse error for Vehicle Identification Request Vin payload type.
    #[error("vehicle identification request with vin payload parse failure")]
    VehicleIdentificationRequestVinError(#[from] VehicleIdentificationRequestVinError),

    /// Urecognised payload type in buffer.
    #[error("invalid payload type")]
    InvalidPayloadType,
}

/// Parse error for Alive Check Response payload type.
#[derive(Error, Debug, PartialEq)]
pub enum AliveCheckResponseError {
    /// The length of the buffer passed is too short to be parsed into this payload
    /// type.
    #[error("length of bytes is too short")]
    InvalidLength,

    /// The parser attempted to access a byte which was not present.
    #[error("invalid index range supplied")]
    InvalidIndexRange,
}

/// Parse error for Diagnostic Message Ack payload type.
#[derive(Error, Debug, PartialEq)]
pub enum DiagnosticMessageAckError {
    /// The length of the buffer passed is too short to be parsed into this payload
    /// type.
    #[error("length of bytes is too short")]
    InvalidLength,

    /// The parser attempted to access a byte which was not present.
    #[error("invalid index range supplied")]
    InvalidIndexRange,

    /// Invalid Ack Code passed to the parser.
    #[error("invalid acknowledgement code")]
    InvalidAckCode,
}

/// Parse error for Diagnostic Message Nack payload type.
#[derive(Error, Debug, PartialEq)]
pub enum DiagnosticMessageNackError {
    /// The length of the buffer passed is too short to be parsed into this payload
    /// type.
    #[error("length of bytes is too short")]
    InvalidLength,

    /// The parser attempted to access a byte which was not present.
    #[error("invalid index range supplied")]
    InvalidIndexRange,

    /// Invalid Nack Code passed to the parser.
    #[error("invalid negative acknowledgement code")]
    InvalidNackCode,
}

/// Parse error for Diagnostic Message payload type.
#[derive(Error, Debug, PartialEq)]
pub enum DiagnosticMessageError {
    /// The length of the buffer passed is too short to be parsed into this payload
    /// type.
    #[error("length of bytes is too short")]
    InvalidLength,

    /// The parser attempted to access a byte which was not present.
    #[error("invalid index range supplied")]
    InvalidIndexRange,
}

/// Parse error for Entity Status Response payload type.
#[derive(Error, Debug, PartialEq)]
pub enum EntityStatusResponseError {
    /// The length of the buffer passed is too short to be parsed into this payload
    /// type.
    #[error("length of bytes is too short")]
    InvalidLength,

    /// The parser attempted to access a byte which was not present.
    #[error("invalid index range supplied")]
    InvalidIndexRange,

    /// Invalid Node Type passed to the parser.
    #[error("invalid node type")]
    InvalidNodeType,
}

/// Parse error for Generic Nack payload type.
#[derive(Error, Debug, PartialEq)]
pub enum GenericNackError {
    /// The length of the buffer passed is too short to be parsed into this payload
    /// type.
    #[error("length of bytes is too short")]
    InvalidLength,

    /// The parser attempted to access a byte which was not present.
    #[error("invalid index range supplied")]
    InvalidIndexRange,

    /// Invalid Nack Code passed to the parser.
    #[error("invalid nack code")]
    InvalidNackCode,
}

/// Parse error for Power Information Response payload type.
#[derive(Error, Debug, PartialEq)]
pub enum PowerInformationResponseError {
    /// The length of the buffer passed is too short to be parsed into this payload
    /// type.
    #[error("length of bytes is too short")]
    InvalidLength,

    /// The parser attempted to access a byte which was not present.
    #[error("invalid index range supplied")]
    InvalidIndexRange,

    /// Invalid Power Mode passed to the parser.
    #[error("powermode not supported")]
    InvalidPowerMode,
}

/// Parse error for Routing Activation Request payload type.
#[derive(Error, Debug, PartialEq)]
pub enum RoutingActivationRequestError {
    /// The length of the buffer passed is too short to be parsed into this payload
    /// type.
    #[error("length of bytes is too short")]
    InvalidLength,

    /// The parser attempted to access a byte which was not present.
    #[error("invalid index range supplied")]
    InvalidIndexRange,

    /// Invalid Activation Type passed to the parser.
    #[error("activation type not supported")]
    InvalidActivationType,
}

/// Parse error for Routing Activation Response payload type.
#[derive(Error, Debug, PartialEq)]
pub enum RoutingActivationResponseError {
    /// The length of the buffer passed is too short to be parsed into this payload
    /// type.
    #[error("length of bytes is too short")]
    InvalidLength,

    /// The parser attempted to access a byte which was not present.
    #[error("invalid index range supplied")]
    InvalidIndexRange,

    /// Invalid Activation Code passed to the parser.
    #[error("activation code not supported")]
    InvalidActivationCode,
}

/// Parse error for Vehicle Announcement Message payload type.
#[derive(Error, Debug, PartialEq)]
pub enum VehicleAnnouncementMessageError {
    /// The length of the buffer passed is too short to be parsed into this payload
    /// type.
    #[error("length of bytes is too short")]
    InvalidLength,

    /// The parser attempted to access a byte which was not present.
    #[error("invalid index range supplied")]
    InvalidIndexRange,

    /// Invalid Action Code passed to the parser.
    #[error("action code not supported")]
    InvalidActionCode,
}

/// Parse error for Vehicle Identification Request Eid payload type.
#[derive(Error, Debug, PartialEq)]
pub enum VehicleIdentificationRequestEidError {
    /// The length of the buffer passed is too short to be parsed into this payload
    /// type.
    #[error("length of bytes is too short")]
    InvalidLength,

    /// The parser attempted to access a byte which was not present.
    #[error("invalid index range supplied")]
    InvalidIndexRange,
}

/// Parse error for Vehicle Identification Request Vin payload type.
#[derive(Error, Debug, PartialEq)]
pub enum VehicleIdentificationRequestVinError {
    /// The length of the buffer passed is too short to be parsed into this payload
    /// type.
    #[error("length of bytes is too short")]
    InvalidLength,

    /// The parser attempted to access a byte which was not present.
    #[error("invalid index range supplied")]
    InvalidIndexRange,
}
