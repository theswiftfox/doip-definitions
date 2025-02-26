use crate::{
    definitions::DOIP_ALIVE_CHECK_RESPONSE_SOURCE_LEN,
    payload::{AliveCheckResponse, DoipPayload},
};

pub fn handle_alive_check_response(slice: &[u8]) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; DOIP_ALIVE_CHECK_RESPONSE_SOURCE_LEN] = match slice.try_into() {
        Ok(bytes) => bytes,
        Err(_) => return Err("Invalid AliveCheckResponse length."),
    };
    Ok(DoipPayload::AliveCheckResponse(AliveCheckResponse::from(
        payload_bytes,
    )))
}

#[cfg(test)]
mod tests {
    use crate::{
        definitions::DOIP_ALIVE_CHECK_RESPONSE_SOURCE_LEN, payload::AliveCheckResponse,
        utils::alive_check_response::handle_alive_check_response,
    };

    #[test]
    fn test_handle_alive_check_response_pass() {
        let payload: [u8; DOIP_ALIVE_CHECK_RESPONSE_SOURCE_LEN] =
            <[u8; DOIP_ALIVE_CHECK_RESPONSE_SOURCE_LEN]>::from(AliveCheckResponse {
                source_address: [0x01, 0x02],
            });

        let result = handle_alive_check_response(&payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_alive_check_response_invalid_length() {
        let payload = &[0x00];
        let res = handle_alive_check_response(payload);

        assert_eq!(res.unwrap_err(), "Invalid AliveCheckResponse length.")
    }
}
