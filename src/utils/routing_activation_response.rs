use crate::{
    definitions::DOIP_ROUTING_ACTIVATION_RES_LEN,
    payload::{DoipPayload, RoutingActivationResponse},
};

pub fn handle_routing_activation_response(
    slice: &[u8],
) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; DOIP_ROUTING_ACTIVATION_RES_LEN] = match slice.try_into() {
        Ok(bytes) => bytes,
        Err(_) => return Err("Invalid RoutingActivationResponse length."),
    };

    match RoutingActivationResponse::try_from(payload_bytes) {
        Ok(bytes) => Ok(DoipPayload::RoutingActivationResponse(bytes)),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        definitions::DOIP_ROUTING_ACTIVATION_RES_LEN,
        payload::{ActivationCode, RoutingActivationResponse},
        utils::routing_activation_response::handle_routing_activation_response,
    };

    #[test]
    fn test_handle_routing_activation_response_pass() {
        let payload: [u8; DOIP_ROUTING_ACTIVATION_RES_LEN] =
            <[u8; DOIP_ROUTING_ACTIVATION_RES_LEN]>::from(RoutingActivationResponse {
                logical_address: [0xff, 0xff],
                source_address: [0xff, 0xff],
                activation_code: ActivationCode::ActivatedConfirmationRequired,
                buffer: [0x00, 0x00, 0x00, 0x00],
            });

        let result = handle_routing_activation_response(&payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_routing_activation_response_fail() {
        let payload: [u8; DOIP_ROUTING_ACTIVATION_RES_LEN] =
            [0x01, 0x02, 0x03, 0x04, 0x45, 0x00, 0x00, 0x00, 0x00];

        let result = handle_routing_activation_response(&payload);

        assert_eq!(result.unwrap_err(), "Invalid ActivationCode.");
    }

    #[test]
    fn test_handle_routing_activation_response_invalid_length() {
        let payload = &[0x00];
        let res = handle_routing_activation_response(payload);

        assert_eq!(
            res.unwrap_err(),
            "Invalid RoutingActivationResponse length."
        )
    }
}
