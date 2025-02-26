use crate::payload::{DoipPayload, EntityStatusRequest};

pub fn handle_entity_status_request(slice: &[u8]) -> Result<DoipPayload<'static>, &'static str> {
    match slice.len() {
        0 => Ok(DoipPayload::EntityStatusRequest(EntityStatusRequest::from(
            [],
        ))),
        _ => Err("Invalid EntityStatusRequest length."),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        payload::EntityStatusRequest, utils::entity_status_request::handle_entity_status_request,
    };

    #[test]
    fn test_handle_entity_status_request_pass() {
        let payload: [u8; 0] = <[u8; 0]>::from(EntityStatusRequest {});

        let result = handle_entity_status_request(&payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_entity_status_request_invalid_length() {
        let payload = &[0x00];
        let res = handle_entity_status_request(payload);

        assert_eq!(res.unwrap_err(), "Invalid EntityStatusRequest length.")
    }
}
