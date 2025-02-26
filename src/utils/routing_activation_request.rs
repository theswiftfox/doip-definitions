use crate::{
    definitions::{DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN, DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN},
    payload::{DoipPayload, RoutingActivationRequest},
};

pub fn handle_routing_activation_request(
    slice: &[u8],
) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN
        + 1
        + DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN] = match slice.try_into() {
        Ok(bytes) => bytes,
        Err(_) => return Err("Invalid RoutingActivationRequest length."),
    };

    match RoutingActivationRequest::try_from(payload_bytes) {
        Ok(bytes) => Ok(DoipPayload::RoutingActivationRequest(bytes)),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        definitions::DOIP_ROUTING_ACTIVATION_REQ_LEN,
        payload::{ActivationType, RoutingActivationRequest},
        utils::routing_activation_request::handle_routing_activation_request,
    };

    #[test]
    fn test_handle_routing_activation_request_pass() {
        let payload: [u8; DOIP_ROUTING_ACTIVATION_REQ_LEN] =
            <[u8; DOIP_ROUTING_ACTIVATION_REQ_LEN]>::from(RoutingActivationRequest {
                source_address: [0x01, 0x02],
                activation_type: ActivationType::Default,
                buffer: [0x00, 0x00, 0x00, 0x00],
            });

        let result = handle_routing_activation_request(&payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_routing_activation_request_fail() {
        let payload: [u8; DOIP_ROUTING_ACTIVATION_REQ_LEN] =
            [0x01, 0x02, 0x45, 0x00, 0x00, 0x00, 0x00];

        let result = handle_routing_activation_request(&payload);

        assert_eq!(result.unwrap_err(), "Invalid ActivationType.");
    }

    #[test]
    fn test_handle_routing_activation_request_invalid_length() {
        let payload = &[0x00];
        let res = handle_routing_activation_request(payload);

        assert_eq!(res.unwrap_err(), "Invalid RoutingActivationRequest length.")
    }
}
