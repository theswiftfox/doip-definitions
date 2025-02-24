use crate::definitions::DOIP_COMMON_EID_LEN;

/// Requests a `VehicleAnnouncementMessage` from entities with the same EID
///
/// Matches `DoIP` entities with the same EID for response to the request.
#[derive(Copy, Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use crate::definitions::DOIP_COMMON_EID_LEN;

    use super::VehicleIdentificationRequestEid;

    #[test]
    fn test_from_bytes() {
        for n in u8::MIN..=u8::MAX {
            let eid_bytes: [u8; DOIP_COMMON_EID_LEN] = [n, n, n, n, n, n];
            let vehicle_identification_req_eid = VehicleIdentificationRequestEid::from(eid_bytes);

            assert_eq!(eid_bytes, vehicle_identification_req_eid.eid)
        }
    }

    #[test]
    fn test_from_vehicle_identification_req_eid() {
        for n in u8::MIN..=u8::MAX {
            let eid_bytes: [u8; DOIP_COMMON_EID_LEN] = [n, n, n, n, n, n];

            let u = <[u8; DOIP_COMMON_EID_LEN]>::from(VehicleIdentificationRequestEid {
                eid: eid_bytes,
            });

            assert_eq!(u, eid_bytes);
        }
    }
}
