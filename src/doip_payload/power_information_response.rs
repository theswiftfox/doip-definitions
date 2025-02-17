use crate::payload::PowerMode;

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

impl From<PowerInformationResponse> for [u8; 1] {
    fn from(power_information_response: PowerInformationResponse) -> Self {
        [power_information_response.power_mode as u8]
    }
}

impl TryFrom<[u8; 1]> for PowerInformationResponse {
    type Error = &'static str;

    fn try_from(value: [u8; 1]) -> Result<Self, Self::Error> {
        let power_mode = PowerMode::try_from(value[0])?;
        Ok(PowerInformationResponse { power_mode })
    }
}
