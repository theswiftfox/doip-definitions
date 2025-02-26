use crate::{
    definitions::DOIP_ENTITY_STATUS_RESPONSE_LEN,
    payload::{DoipPayload, EntityStatusResponse},
};

pub fn handle_entity_status_response(slice: &[u8]) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; DOIP_ENTITY_STATUS_RESPONSE_LEN] = match slice.try_into() {
        Ok(bytes) => bytes,
        Err(_) => return Err("Invalid EntityStatusResponse length."),
    };

    match EntityStatusResponse::try_from(payload_bytes) {
        Ok(bytes) => Ok(DoipPayload::EntityStatusResponse(bytes)),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        definitions::DOIP_ENTITY_STATUS_RESPONSE_LEN,
        payload::{EntityStatusResponse, NodeType},
        utils::entity_status_response::handle_entity_status_response,
    };

    #[test]
    fn test_handle_entity_status_response_pass() {
        let payload: [u8; DOIP_ENTITY_STATUS_RESPONSE_LEN] =
            <[u8; DOIP_ENTITY_STATUS_RESPONSE_LEN]>::from(EntityStatusResponse {
                node_type: NodeType::DoipGateway,
                max_concurrent_sockets: [0x01],
                currently_open_sockets: [0x02],
                max_data_size: [0x03, 0x04, 0x05, 0x06],
            });

        let result = handle_entity_status_response(&payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_entity_status_response_fail() {
        let payload: [u8; DOIP_ENTITY_STATUS_RESPONSE_LEN] =
            [0xff, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06];

        let result = handle_entity_status_response(&payload);

        assert_eq!(result.unwrap_err(), "Invalid NodeType.");
    }

    #[test]
    fn test_handle_entity_status_response_invalid_length() {
        let payload = &[0x00];
        let res = handle_entity_status_response(payload);

        assert_eq!(res.unwrap_err(), "Invalid EntityStatusResponse length.")
    }
}
