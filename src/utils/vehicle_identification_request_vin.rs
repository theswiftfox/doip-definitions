use crate::{
    definitions::DOIP_COMMON_VIN_LEN,
    payload::{DoipPayload, VehicleIdentificationRequestVin},
};

pub fn handle_vehicle_identification_request_vin(
    slice: &[u8],
) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; DOIP_COMMON_VIN_LEN] = match slice.try_into() {
        Ok(bytes) => bytes,
        Err(_) => return Err("Invalid VehicleIdentificationRequestVin length."),
    };
    Ok(DoipPayload::VehicleIdentificationRequestVin(
        VehicleIdentificationRequestVin::from(payload_bytes),
    ))
}

#[cfg(test)]
mod tests {
    use crate::{
        definitions::DOIP_COMMON_VIN_LEN,
        utils::vehicle_identification_request_vin::handle_vehicle_identification_request_vin,
    };

    #[test]
    fn test_handle_vehicle_identification_request_vin_pass() {
        let payload = &[0u8; DOIP_COMMON_VIN_LEN];
        let res = handle_vehicle_identification_request_vin(payload);

        assert!(res.is_ok())
    }

    #[test]
    fn test_handle_vehicle_identification_request_vin_invalid_length() {
        let payload = &[0x00];
        let res = handle_vehicle_identification_request_vin(payload);

        assert_eq!(
            res.unwrap_err(),
            "Invalid VehicleIdentificationRequestVin length."
        )
    }
}
