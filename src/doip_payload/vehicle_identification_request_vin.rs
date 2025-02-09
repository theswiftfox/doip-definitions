use crate::{
    definitions::DOIP_COMMON_VIN_LEN,
    header::{DoipPayload, PayloadType},
};

/// Requests a `VehicleAnnouncementMessage` from entities with the same VIN
///
/// Matches `DoIP` entities with the same VIN for response to the request.
#[derive(Copy, Clone, Debug)]
pub struct VehicleIdentificationRequestVin {
    /// Vehicle Identification Number
    pub vin: [u8; DOIP_COMMON_VIN_LEN],
}

impl DoipPayload for VehicleIdentificationRequestVin {
    fn payload_type(&self) -> PayloadType {
        PayloadType::VehicleIdentificationRequestVin
    }
}
