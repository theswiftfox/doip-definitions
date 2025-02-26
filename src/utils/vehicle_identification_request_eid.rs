use crate::{
    definitions::DOIP_COMMON_EID_LEN,
    payload::{DoipPayload, VehicleIdentificationRequestEid},
};

pub fn handle_vehicle_identification_request_eid(
    slice: &[u8],
) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; DOIP_COMMON_EID_LEN] = match slice.try_into() {
        Ok(bytes) => bytes,
        Err(_) => return Err("Invalid VehicleIdentificationRequestEid length."),
    };
    Ok(DoipPayload::VehicleIdentificationRequestEid(
        VehicleIdentificationRequestEid::from(payload_bytes),
    ))
}

#[cfg(test)]
mod tests {
    use crate::utils::vehicle_identification_request_eid::handle_vehicle_identification_request_eid;

    #[test]
    fn test_handle_vehicle_identification_request_eid_pass() {
        let payload = &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let res = handle_vehicle_identification_request_eid(payload);

        assert!(res.is_ok())
    }

    #[test]
    fn test_handle_vehicle_identification_request_eid_invalid_length() {
        let payload = &[0x00];
        let res = handle_vehicle_identification_request_eid(payload);

        assert_eq!(
            res.unwrap_err(),
            "Invalid VehicleIdentificationRequestEid length."
        )
    }
}
