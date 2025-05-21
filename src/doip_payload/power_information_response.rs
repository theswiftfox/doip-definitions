use crate::payload::PowerMode;

/// Expected reponse from `PowerInformationRequest`.
///
/// Containing details of the target of the `PowerInformationRequest`, the
/// `EntityStatusResponse` provides the program with details pertaining to the
/// active power mode status of the entity.

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PowerInformationResponse {
    /// Possible power modes available
    pub power_mode: PowerMode,
}
