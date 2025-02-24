/// Used to request a `VehicleAnnouncement` from all available `DoIP` entities
/// on the network.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VehicleIdentificationRequest {}

impl From<VehicleIdentificationRequest> for [u8; 0] {
    fn from(vehicle_identification_request: VehicleIdentificationRequest) -> Self {
        let _ = vehicle_identification_request;
        []
    }
}

impl From<[u8; 0]> for VehicleIdentificationRequest {
    fn from(value: [u8; 0]) -> Self {
        match value {
            [] => VehicleIdentificationRequest {},
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VehicleIdentificationRequest;

    #[test]
    fn test_from_bytes() {
        let n = [];
        let vehicle_identification_request = VehicleIdentificationRequest::from(n);
        assert_eq!(
            vehicle_identification_request,
            VehicleIdentificationRequest {}
        )
    }
    #[test]
    fn test_from_vehicle_id_req() {
        let u = <[u8; 0]>::from(VehicleIdentificationRequest {});
        assert_eq!(u, []);
    }
}
