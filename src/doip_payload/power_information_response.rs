use crate::{
    header::{DoipPayload, PayloadType},
    message::PowerMode,
};

/// Expected reponse from `PowerInformationRequest`.
///
/// Containing details of the target of the `PowerInformationRequest`, the
/// `EntityStatusResponse` provides the program with details pertaining to the
/// active power mode status of the entity.
#[derive(Copy, Clone, Debug)]
pub struct PowerInformationResponse {
    /// Possible power modes available
    pub power_mode: PowerMode,
}

impl DoipPayload for PowerInformationResponse {
    fn payload_type(&self) -> PayloadType {
        PayloadType::PowerInformationResponse
    }
}
