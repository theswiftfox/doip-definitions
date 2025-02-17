use crate::{
    definitions::{
        DOIP_ROUTING_ACTIVATION_RES_CODE_LEN, DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN,
        DOIP_ROUTING_ACTIVATION_RES_ISO_LEN, DOIP_ROUTING_ACTIVATION_RES_LEN,
        DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN,
    },
    payload::ActivationCode,
};

/// A response to the `RoutingActivationRequest`.
///
/// Contains the logical address of the recieved `DoIP` entity along with the activation code.
#[derive(Copy, Clone, Debug)]
pub struct RoutingActivationResponse {
    /// Logical address of requested entity
    pub logical_address: [u8; DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN],

    /// Source address of response entity
    pub source_address: [u8; DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN],

    /// Activation Code
    pub activation_code: ActivationCode,

    /// ISO reserved buffer
    pub buffer: [u8; DOIP_ROUTING_ACTIVATION_RES_ISO_LEN],
}

impl From<RoutingActivationResponse> for [u8; DOIP_ROUTING_ACTIVATION_RES_LEN] {
    fn from(routing_activation_response: RoutingActivationResponse) -> Self {
        let logical_address = routing_activation_response.logical_address;
        let source_address = routing_activation_response.source_address;
        let activation_code = [u8::from(routing_activation_response.activation_code)];
        let res_buffer = routing_activation_response.buffer;

        let mut buffer = [0; DOIP_ROUTING_ACTIVATION_RES_LEN];
        let mut offset = 0;

        buffer[offset..offset + DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN]
            .copy_from_slice(&logical_address);
        offset += DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN;

        buffer[offset..offset + DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN]
            .copy_from_slice(&source_address);
        offset += DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN;

        buffer[offset] = activation_code[0];
        offset += DOIP_ROUTING_ACTIVATION_RES_CODE_LEN;

        buffer[offset..offset + DOIP_ROUTING_ACTIVATION_RES_ISO_LEN].copy_from_slice(&res_buffer);

        buffer
    }
}

impl TryFrom<[u8; DOIP_ROUTING_ACTIVATION_RES_LEN]> for RoutingActivationResponse {
    type Error = &'static str;

    fn try_from(value: [u8; DOIP_ROUTING_ACTIVATION_RES_LEN]) -> Result<Self, Self::Error> {
        let (logical_address_slice, rest) = value.split_at(DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN);
        let (source_address_slice, rest) = rest.split_at(DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN);
        let (activation_code_slice, rest) = rest.split_at(DOIP_ROUTING_ACTIVATION_RES_CODE_LEN);
        let (buffer_slice, _) = rest.split_at(DOIP_ROUTING_ACTIVATION_RES_ISO_LEN);

        let logical_address: [u8; DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN] = logical_address_slice
            .try_into()
            .map_err(|_| "Invalid logical address length")?;

        let source_address: [u8; DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN] = source_address_slice
            .try_into()
            .map_err(|_| "Invalid source address length")?;

        let activation_code = ActivationCode::try_from(activation_code_slice[0])?;

        let buffer: [u8; DOIP_ROUTING_ACTIVATION_RES_ISO_LEN] = buffer_slice
            .try_into()
            .map_err(|_| "Invalid buffer length")?;

        Ok(RoutingActivationResponse {
            logical_address,
            source_address,
            activation_code,
            buffer,
        })
    }
}
