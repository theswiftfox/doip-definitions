/// Used to request a `VehicleAnnouncement` from all available `DoIP` entities
/// on the network.
#[derive(Copy, Clone, Debug)]
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
