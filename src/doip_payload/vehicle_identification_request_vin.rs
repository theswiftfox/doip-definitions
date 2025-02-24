use crate::definitions::DOIP_COMMON_VIN_LEN;

/// Requests a `VehicleAnnouncementMessage` from entities with the same VIN
///
/// Matches `DoIP` entities with the same VIN for response to the request.
#[derive(Copy, Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use crate::definitions::DOIP_COMMON_VIN_LEN;

    use super::VehicleIdentificationRequestVin;

    #[test]
    fn test_from_bytes() {
        for n in u8::MIN..=u8::MAX {
            let vin_bytes: [u8; DOIP_COMMON_VIN_LEN] =
                [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n];
            let vehicle_identification_req_vin = VehicleIdentificationRequestVin::from(vin_bytes);

            assert_eq!(vin_bytes, vehicle_identification_req_vin.vin)
        }
    }

    #[test]
    fn test_from_vehicle_identification_req_vin() {
        for n in u8::MIN..=u8::MAX {
            let vin_bytes: [u8; DOIP_COMMON_VIN_LEN] =
                [n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n, n];

            let u = <[u8; DOIP_COMMON_VIN_LEN]>::from(VehicleIdentificationRequestVin {
                vin: vin_bytes,
            });

            assert_eq!(u, vin_bytes);
        }
    }
}
