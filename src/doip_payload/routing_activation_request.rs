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
#[derive(Copy, Clone, Debug)]
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

        let source_address: [u8; DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN] = source_slice
            .try_into()
            .map_err(|_| "Invalid source address length")?;

        let activation_type = ActivationType::try_from(activation_type_slice[0])?;

        let buffer: [u8; DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN] = buffer_slice
            .try_into()
            .map_err(|_| "Invalid buffer length")?;

        Ok(RoutingActivationRequest {
            source_address,
            activation_type,
            buffer,
        })
    }
}
