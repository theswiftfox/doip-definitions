use crate::{
    definitions::DOIP_POWER_MODE_LEN,
    payload::{DoipPayload, PowerInformationResponse},
};

pub fn handle_power_information_response(
    slice: &[u8],
) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; DOIP_POWER_MODE_LEN] = match slice.try_into() {
        Ok(bytes) => bytes,
        Err(_) => return Err("Invalid PowerInformationResponse length."),
    };

    match PowerInformationResponse::try_from(payload_bytes) {
        Ok(bytes) => Ok(DoipPayload::PowerInformationResponse(bytes)),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        definitions::DOIP_POWER_MODE_LEN,
        payload::{PowerInformationResponse, PowerMode},
        utils::power_information_response::handle_power_information_response,
    };

    #[test]
    fn test_handle_power_information_response_pass() {
        let payload: [u8; DOIP_POWER_MODE_LEN] =
            <[u8; DOIP_POWER_MODE_LEN]>::from(PowerInformationResponse {
                power_mode: PowerMode::Ready,
            });

        let result = handle_power_information_response(&payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_power_information_response_fail() {
        let payload: [u8; DOIP_POWER_MODE_LEN] = [0xff];

        let result = handle_power_information_response(&payload);

        assert_eq!(result.unwrap_err(), "Invalid PowerMode.");
    }

    #[test]
    fn test_handle_power_information_response_invalid_length() {
        let payload = &[];
        let res = handle_power_information_response(payload);

        assert_eq!(res.unwrap_err(), "Invalid PowerInformationResponse length.")
    }
}
