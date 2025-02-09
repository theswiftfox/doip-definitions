use crate::{
    definitions::DOIP_COMMON_EID_LEN,
    header::{DoipPayload, PayloadType},
};

/// Requests a `VehicleAnnouncementMessage` from entities with the same EID
///
/// Matches `DoIP` entities with the same EID for response to the request.
#[derive(Copy, Clone, Debug)]
pub struct VehicleIdentificationRequestEid {
    /// Entity Identification
    pub eid: [u8; DOIP_COMMON_EID_LEN],
}

impl DoipPayload for VehicleIdentificationRequestEid {
    fn payload_type(&self) -> PayloadType {
        PayloadType::VehicleIdentificationRequestEid
    }
}
