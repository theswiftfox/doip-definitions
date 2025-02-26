use crate::payload::{DoipPayload, VehicleIdentificationRequest};

pub fn handle_vehicle_identification_request(
    slice: &[u8],
) -> Result<DoipPayload<'static>, &'static str> {
    match slice.len() {
        0 => Ok(DoipPayload::VehicleIdentificationRequest(
            VehicleIdentificationRequest::from([]),
        )),
        _ => Err("Invalid VehicleIdentificationRequest length."),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        payload::{DoipPayload, VehicleIdentificationRequest},
        utils::vehicle_identification_request::handle_vehicle_identification_request,
    };

    #[test]
    fn test_handle_vehicle_identification_request_pass() {
        let payload = &[];
        let res = handle_vehicle_identification_request(payload);

        assert_eq!(
            res.unwrap(),
            DoipPayload::VehicleIdentificationRequest(VehicleIdentificationRequest {})
        )
    }

    #[test]
    fn test_handle_vehicle_identification_request_invalid_length() {
        let payload = &[0x00];
        let res = handle_vehicle_identification_request(payload);

        assert_eq!(
            res.unwrap_err(),
            "Invalid VehicleIdentificationRequest length."
        )
    }
}
