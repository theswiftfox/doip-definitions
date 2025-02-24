/// Requests the power mode status of a `DoIP` Entity.
#[derive(Copy, Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::PowerInformationRequest;

    #[test]
    fn test_from_bytes() {
        let n = [];
        let power_information_request = PowerInformationRequest::from(n);
        assert_eq!(power_information_request, PowerInformationRequest {})
    }
    #[test]
    fn test_from_power_info_req() {
        let u = <[u8; 0]>::from(PowerInformationRequest {});
        assert_eq!(u, []);
    }
}
