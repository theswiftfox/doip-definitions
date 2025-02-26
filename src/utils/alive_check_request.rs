use crate::payload::{AliveCheckRequest, DoipPayload};

pub fn handle_alive_check_request(slice: &[u8]) -> Result<DoipPayload<'static>, &'static str> {
    match slice.len() {
        0 => Ok(DoipPayload::AliveCheckRequest(AliveCheckRequest::from([]))),
        _ => Err("Invalid AliveCheckRequest length."),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        payload::AliveCheckRequest, utils::alive_check_request::handle_alive_check_request,
    };

    #[test]
    fn test_handle_alive_check_request_pass() {
        let payload: [u8; 0] = <[u8; 0]>::from(AliveCheckRequest {});

        let result = handle_alive_check_request(&payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_alive_check_request_invalid_length() {
        let payload = &[0x00];
        let res = handle_alive_check_request(payload);

        assert_eq!(res.unwrap_err(), "Invalid AliveCheckRequest length.")
    }
}
