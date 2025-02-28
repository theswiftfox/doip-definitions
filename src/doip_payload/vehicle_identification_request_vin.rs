use crate::definitions::DOIP_COMMON_VIN_LEN;

/// Requests a `VehicleAnnouncementMessage` from entities with the same VIN
///
/// Matches `DoIP` entities with the same VIN for response to the request.
#[cfg_attr(feature = "std", pyo3::pyclass)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VehicleIdentificationRequestVin {
    /// Vehicle Identification Number
    pub vin: [u8; DOIP_COMMON_VIN_LEN],
}
