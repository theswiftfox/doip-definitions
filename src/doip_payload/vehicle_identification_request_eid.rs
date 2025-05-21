use crate::definitions::DOIP_COMMON_EID_LEN;

/// Requests a `VehicleAnnouncementMessage` from entities with the same EID
///
/// Matches `DoIP` entities with the same EID for response to the request.

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VehicleIdentificationRequestEid {
    /// Entity Identification
    pub eid: [u8; DOIP_COMMON_EID_LEN],
}
