use crate::definitions::DOIP_COMMON_VIN_LEN;

/// Requests a `VehicleAnnouncementMessage` from entities with the same VIN
///
/// Matches `DoIP` entities with the same VIN for response to the request.
#[derive(Copy, Clone, Debug)]
pub struct VehicleIdentificationRequestVin {
    /// Vehicle Identification Number
    pub vin: [u8; DOIP_COMMON_VIN_LEN],
}

impl From<VehicleIdentificationRequestVin> for [u8; DOIP_COMMON_VIN_LEN] {
    fn from(vehicle_identification_request_vin: VehicleIdentificationRequestVin) -> Self {
        vehicle_identification_request_vin.vin
    }
}

impl From<[u8; DOIP_COMMON_VIN_LEN]> for VehicleIdentificationRequestVin {
    fn from(value: [u8; DOIP_COMMON_VIN_LEN]) -> Self {
        VehicleIdentificationRequestVin { vin: value }
    }
}
