use crate::definitions::DOIP_COMMON_EID_LEN;

/// Requests a `VehicleAnnouncementMessage` from entities with the same EID
///
/// Matches `DoIP` entities with the same EID for response to the request.
#[derive(Copy, Clone, Debug)]
pub struct VehicleIdentificationRequestEid {
    /// Entity Identification
    pub eid: [u8; DOIP_COMMON_EID_LEN],
}

impl From<VehicleIdentificationRequestEid> for [u8; DOIP_COMMON_EID_LEN] {
    fn from(vehicle_identification_request_eid: VehicleIdentificationRequestEid) -> Self {
        vehicle_identification_request_eid.eid
    }
}

impl From<[u8; DOIP_COMMON_EID_LEN]> for VehicleIdentificationRequestEid {
    fn from(value: [u8; DOIP_COMMON_EID_LEN]) -> Self {
        VehicleIdentificationRequestEid { eid: value }
    }
}
