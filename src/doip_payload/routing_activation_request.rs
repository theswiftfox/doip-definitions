use crate::{
    definitions::{
        DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN, DOIP_ROUTING_ACTIVATION_REQ_LEN,
        DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN, DOIP_ROUTING_ACTIVATION_REQ_TYPE_LEN_V2,
    },
    payload::ActivationType,
};

/// Request for routing activation.
///
/// Usually routing activation is required for `DoipMessage` passing to a `DoIP` server,
/// the `RoutingActivationRequest` details the activation type required.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RoutingActivationRequest {
    /// Source address of the requesting entity
    pub source_address: [u8; DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN],

    /// Activation type
    pub activation_type: ActivationType,

    /// ISO reserved buffer, currently left empty
    pub buffer: [u8; DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN],
}

impl From<RoutingActivationRequest> for [u8; DOIP_ROUTING_ACTIVATION_REQ_LEN] {
    fn from(routing_activation_request: RoutingActivationRequest) -> Self {
        let source_address = routing_activation_request.source_address;
        let activation_type = [u8::from(routing_activation_request.activation_type)];
        let req_buffer = routing_activation_request.buffer;

        let mut buffer = [0; DOIP_ROUTING_ACTIVATION_REQ_LEN];
        let mut offset = 0;

        buffer[offset..offset + DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN]
            .copy_from_slice(&source_address);
        offset += DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN;

        buffer[offset] = activation_type[0];
        offset += DOIP_ROUTING_ACTIVATION_REQ_TYPE_LEN_V2;

        buffer[offset..offset + DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN].copy_from_slice(&req_buffer);

        buffer
    }
}

impl TryFrom<[u8; DOIP_ROUTING_ACTIVATION_REQ_LEN]> for RoutingActivationRequest {
    type Error = &'static str;

    fn try_from(value: [u8; DOIP_ROUTING_ACTIVATION_REQ_LEN]) -> Result<Self, Self::Error> {
        let (source_slice, rest) = value.split_at(DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN);
        let (activation_type_slice, rest) = rest.split_at(DOIP_ROUTING_ACTIVATION_REQ_TYPE_LEN_V2);
        let (buffer_slice, _) = rest.split_at(DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN);

        let mut source_address = [0u8; DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN];
        source_address.copy_from_slice(source_slice);

        let activation_type = ActivationType::try_from(activation_type_slice[0])?;

        let mut buffer = [0u8; DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN];
        buffer.copy_from_slice(buffer_slice);

        Ok(RoutingActivationRequest {
            source_address,
            activation_type,
            buffer,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{definitions::DOIP_ROUTING_ACTIVATION_REQ_LEN, payload::ActivationType};

    use super::RoutingActivationRequest;

    #[test]
    fn test_try_from_bytes() {
        for a in u8::MIN..=u8::MAX {
            let routing_req =
                RoutingActivationRequest::try_from([0x01, 0x02, a, 0x03, 0x04, 0x05, 0x06]);

            match a {
                0x00 => assert_eq!(
                    routing_req.unwrap(),
                    RoutingActivationRequest {
                        source_address: [0x01, 0x02],
                        activation_type: ActivationType::Default,
                        buffer: [0x03, 0x04, 0x05, 0x06]
                    }
                ),
                0x01 => assert_eq!(
                    routing_req.unwrap(),
                    RoutingActivationRequest {
                        source_address: [0x01, 0x02],
                        activation_type: ActivationType::WwhObd,
                        buffer: [0x03, 0x04, 0x05, 0x06]
                    }
                ),
                0x02 => assert_eq!(
                    routing_req.unwrap(),
                    RoutingActivationRequest {
                        source_address: [0x01, 0x02],
                        activation_type: ActivationType::CentralSecurity,
                        buffer: [0x03, 0x04, 0x05, 0x06]
                    }
                ),
                _ => assert_eq!(routing_req.unwrap_err(), "Invalid ActivationType."),
            };
        }
    }

    #[test]
    fn test_from_from_routing_activiation_req() {
        let routing_activation_req = RoutingActivationRequest {
            source_address: [0x01, 0x02],
            activation_type: ActivationType::Default,
            buffer: [0x00, 0x00, 0x00, 0x00],
        };

        let routing_activation_req_bytes =
            <[u8; DOIP_ROUTING_ACTIVATION_REQ_LEN]>::from(routing_activation_req);
        assert_eq!(
            routing_activation_req_bytes,
            [0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00]
        )
    }
}
