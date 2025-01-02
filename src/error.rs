use thiserror::Error;

use crate::header::payload::{
    AliveCheckResponseParseError, DiagnosticMessageAckError, DiagnosticMessageError,
    DiagnosticMessageNackParseError, EntityStatusResponseError, GenericNackError,
    PowerInformationResponseError, RoutingActivationRequestError, RoutingActivationResponseError,
    VehicleAnnouncementMessageError, VehicleIdentificationRequestEidError,
    VehicleIdentificationRequestVinError,
};

#[derive(Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("empty input")]
    EmptyInput,
    #[error("invalid protocol version")]
    InvalidProtocolVersion,
    #[error("failed protocol check")]
    FailedProtocolCheck,
    #[error("index failure")]
    IndexFailure,
    #[error("incomplete payload")]
    IncompletePayload,
    #[error("payload parse error")]
    PayloadParse(#[from] PayloadError),
}

#[derive(Error, Debug, PartialEq)]
pub enum PayloadError {
    #[error("alive check payload parse failure")]
    AliveCheckResponseParseError(#[from] AliveCheckResponseParseError),
    #[error("diuagnostic message negative acknolwedgement payload parse failure")]
    DiagnosticMessageNackParseError(#[from] DiagnosticMessageNackParseError),
    #[error("diagnostic message acknowledgement payload parse failure")]
    DiagnosticMessageAckError(#[from] DiagnosticMessageAckError),
    #[error("diagnostic message payload parse failure")]
    DiagnosticMessageError(#[from] DiagnosticMessageError),
    #[error("entity status response payload parse failure")]
    EntityStatusResponseError(#[from] EntityStatusResponseError),
    #[error("generic nack payload parse failure")]
    GenericNackError(#[from] GenericNackError),
    #[error("power information response payload parse failure")]
    PowerInformationResponseError(#[from] PowerInformationResponseError),
    #[error("routing activation request payload parse failure")]
    RoutingActivationRequestError(#[from] RoutingActivationRequestError),
    #[error("routing activation response payload parse failure")]
    RoutingActivationResponseError(#[from] RoutingActivationResponseError),
    #[error("vehicle announcement message payload parse failure")]
    VehicleAnnouncementMessageError(#[from] VehicleAnnouncementMessageError),
    #[error("vehicle identification request with eid payload parse failure")]
    VehicleIdentificationRequestEidError(#[from] VehicleIdentificationRequestEidError),
    #[error("vehicle identification request with vin payload parse failure")]
    VehicleIdentificationRequestVinError(#[from] VehicleIdentificationRequestVinError),
    #[error("invalid payload type")]
    InvalidPayloadType,
}
