use crate::payload::{DoipPayload, PowerInformationRequest};

pub fn handle_power_information_request(
    slice: &[u8],
) -> Result<DoipPayload<'static>, &'static str> {
    match slice.len() {
        0 => Ok(DoipPayload::PowerInformationRequest(
            PowerInformationRequest::from([]),
        )),
        _ => Err("Invalid PowerInformationRequest length."),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        payload::PowerInformationRequest,
        utils::power_information_request::handle_power_information_request,
    };

    #[test]
    fn test_handle_power_information_request_pass() {
        let payload: [u8; 0] = <[u8; 0]>::from(PowerInformationRequest {});

        let result = handle_power_information_request(&payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_power_information_request_invalid_length() {
        let payload = &[0x00];
        let res = handle_power_information_request(payload);

        assert_eq!(res.unwrap_err(), "Invalid PowerInformationRequest length.")
    }
}
