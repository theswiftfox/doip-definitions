/// Requests the power mode status of a `DoIP` Entity.
#[derive(Copy, Clone, Debug)]
pub struct PowerInformationRequest {}

impl From<PowerInformationRequest> for [u8; 0] {
    fn from(power_information_request: PowerInformationRequest) -> Self {
        let _ = power_information_request;
        []
    }
}

impl From<[u8; 0]> for PowerInformationRequest {
    fn from(value: [u8; 0]) -> Self {
        match value {
            [] => PowerInformationRequest {},
        }
    }
}
