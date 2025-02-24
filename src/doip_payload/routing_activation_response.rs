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
#[derive(Copy, Clone, Debug, PartialEq)]
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

        let mut logical_address = [0u8; DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN];
        logical_address.copy_from_slice(logical_address_slice);

        let mut source_address = [0u8; DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN];
        source_address.copy_from_slice(source_address_slice);

        let activation_code = ActivationCode::try_from(activation_code_slice[0])?;

        let mut buffer = [0u8; DOIP_ROUTING_ACTIVATION_RES_ISO_LEN];
        buffer.copy_from_slice(buffer_slice);

        Ok(RoutingActivationResponse {
            logical_address,
            source_address,
            activation_code,
            buffer,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::definitions::DOIP_ROUTING_ACTIVATION_RES_LEN;

    use super::{ActivationCode, RoutingActivationResponse};

    #[test]
    fn test_try_from_bytes() {
        for a in u8::MIN..=u8::MAX {
            let routing_res = RoutingActivationResponse::try_from([
                0x01, 0x02, 0x03, 0x04, a, 0x05, 0x06, 0x07, 0x08,
            ]);

            match a {
                0x00 => assert_eq!(
                    routing_res.unwrap(),
                    RoutingActivationResponse {
                        logical_address: [0x01, 0x02],
                        source_address: [0x03, 0x04],
                        activation_code: ActivationCode::DeniedUnknownSourceAddress,
                        buffer: [0x05, 0x06, 0x07, 0x08]
                    }
                ),
                0x01 => {
                    assert_eq!(
                        routing_res.unwrap(),
                        RoutingActivationResponse {
                            logical_address: [0x01, 0x02],
                            source_address: [0x03, 0x04],
                            activation_code: ActivationCode::DeniedTCPSocketsFull,
                            buffer: [0x05, 0x06, 0x07, 0x08]
                        }
                    )
                }
                0x02 => assert_eq!(
                    routing_res.unwrap(),
                    RoutingActivationResponse {
                        logical_address: [0x01, 0x02],
                        source_address: [0x03, 0x04],
                        activation_code: ActivationCode::DeniedTCPSocketAlreadyConnected,
                        buffer: [0x05, 0x06, 0x07, 0x08]
                    }
                ),
                0x03 => assert_eq!(
                    routing_res.unwrap(),
                    RoutingActivationResponse {
                        logical_address: [0x01, 0x02],
                        source_address: [0x03, 0x04],
                        activation_code: ActivationCode::DeniedSourceIsAlreadyActive,
                        buffer: [0x05, 0x06, 0x07, 0x08]
                    }
                ),
                0x04 => assert_eq!(
                    routing_res.unwrap(),
                    RoutingActivationResponse {
                        logical_address: [0x01, 0x02],
                        source_address: [0x03, 0x04],
                        activation_code: ActivationCode::DeniedMissingAuthentication,
                        buffer: [0x05, 0x06, 0x07, 0x08]
                    }
                ),
                0x05 => assert_eq!(
                    routing_res.unwrap(),
                    RoutingActivationResponse {
                        logical_address: [0x01, 0x02],
                        source_address: [0x03, 0x04],
                        activation_code: ActivationCode::DeniedRejectedConfirmation,
                        buffer: [0x05, 0x06, 0x07, 0x08]
                    }
                ),
                0x06 => assert_eq!(
                    routing_res.unwrap(),
                    RoutingActivationResponse {
                        logical_address: [0x01, 0x02],
                        source_address: [0x03, 0x04],
                        activation_code: ActivationCode::DeniedUnsupportedRoutingActivationType,
                        buffer: [0x05, 0x06, 0x07, 0x08]
                    }
                ),
                0x07 => assert_eq!(
                    routing_res.unwrap(),
                    RoutingActivationResponse {
                        logical_address: [0x01, 0x02],
                        source_address: [0x03, 0x04],
                        activation_code: ActivationCode::DeniedRequestEncryptedTLSConnection,
                        buffer: [0x05, 0x06, 0x07, 0x08]
                    }
                ),
                0x08 => {
                    assert_eq!(
                        routing_res.unwrap(),
                        RoutingActivationResponse {
                            logical_address: [0x01, 0x02],
                            source_address: [0x03, 0x04],
                            activation_code: ActivationCode::ReservedByIso13400_08,
                            buffer: [0x05, 0x06, 0x07, 0x08]
                        }
                    )
                }
                0x09 => {
                    assert_eq!(
                        routing_res.unwrap(),
                        RoutingActivationResponse {
                            logical_address: [0x01, 0x02],
                            source_address: [0x03, 0x04],
                            activation_code: ActivationCode::ReservedByIso13400_09,
                            buffer: [0x05, 0x06, 0x07, 0x08]
                        }
                    )
                }
                0x0A => {
                    assert_eq!(
                        routing_res.unwrap(),
                        RoutingActivationResponse {
                            logical_address: [0x01, 0x02],
                            source_address: [0x03, 0x04],
                            activation_code: ActivationCode::ReservedByIso13400_0A,
                            buffer: [0x05, 0x06, 0x07, 0x08]
                        }
                    )
                }
                0x0B => {
                    assert_eq!(
                        routing_res.unwrap(),
                        RoutingActivationResponse {
                            logical_address: [0x01, 0x02],
                            source_address: [0x03, 0x04],
                            activation_code: ActivationCode::ReservedByIso13400_0B,
                            buffer: [0x05, 0x06, 0x07, 0x08]
                        }
                    )
                }
                0x0C => {
                    assert_eq!(
                        routing_res.unwrap(),
                        RoutingActivationResponse {
                            logical_address: [0x01, 0x02],
                            source_address: [0x03, 0x04],
                            activation_code: ActivationCode::ReservedByIso13400_0C,
                            buffer: [0x05, 0x06, 0x07, 0x08]
                        }
                    )
                }
                0x0D => {
                    assert_eq!(
                        routing_res.unwrap(),
                        RoutingActivationResponse {
                            logical_address: [0x01, 0x02],
                            source_address: [0x03, 0x04],
                            activation_code: ActivationCode::ReservedByIso13400_0D,
                            buffer: [0x05, 0x06, 0x07, 0x08]
                        }
                    )
                }
                0x0E => {
                    assert_eq!(
                        routing_res.unwrap(),
                        RoutingActivationResponse {
                            logical_address: [0x01, 0x02],
                            source_address: [0x03, 0x04],
                            activation_code: ActivationCode::ReservedByIso13400_0E,
                            buffer: [0x05, 0x06, 0x07, 0x08]
                        }
                    )
                }
                0x0F => {
                    assert_eq!(
                        routing_res.unwrap(),
                        RoutingActivationResponse {
                            logical_address: [0x01, 0x02],
                            source_address: [0x03, 0x04],
                            activation_code: ActivationCode::ReservedByIso13400_0F,
                            buffer: [0x05, 0x06, 0x07, 0x08]
                        }
                    )
                }
                0x10 => {
                    assert_eq!(
                        routing_res.unwrap(),
                        RoutingActivationResponse {
                            logical_address: [0x01, 0x02],
                            source_address: [0x03, 0x04],
                            activation_code: ActivationCode::SuccessfullyActivated,
                            buffer: [0x05, 0x06, 0x07, 0x08]
                        }
                    )
                }
                0x11 => assert_eq!(
                    routing_res.unwrap(),
                    RoutingActivationResponse {
                        logical_address: [0x01, 0x02],
                        source_address: [0x03, 0x04],
                        activation_code: ActivationCode::ActivatedConfirmationRequired,
                        buffer: [0x05, 0x06, 0x07, 0x08]
                    }
                ),
                _ => assert_eq!(routing_res.unwrap_err(), "Invalid ActivationCode."),
            };
        }
    }

    #[test]
    fn test_from_routing_activation_res() {
        let routing_activation_res = RoutingActivationResponse {
            logical_address: [0x01, 0x02],
            source_address: [0x03, 0x04],
            activation_code: ActivationCode::SuccessfullyActivated,
            buffer: [0x00, 0x00, 0x00, 0x00],
        };

        let routing_activation_res_bytes =
            <[u8; DOIP_ROUTING_ACTIVATION_RES_LEN]>::from(routing_activation_res);
        assert_eq!(
            routing_activation_res_bytes,
            [0x01, 0x02, 0x03, 0x04, 0x10, 0x00, 0x00, 0x00, 0x00]
        );
    }
}
