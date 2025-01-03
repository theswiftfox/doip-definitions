use crate::{
    definitions::{
        DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN, DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN,
        DOIP_ROUTING_ACTIVATION_REQ_TYPE_LEN_V2,
    },
    error::{PayloadError, RoutingActivationRequestError},
    header::{DoipPayload, PayloadType},
    message::ActivationType,
};

/// Request for routing activation.
///
/// Usually routing activation is required for `DoipMessage` passing to a DoIP server,
/// the `RoutingActivationRequest` details the activation type required.
#[derive(Copy, Clone, Debug)]
pub struct RoutingActivationRequest {
    /// Source address of the requesting entity
    pub source_address: [u8; 2],

    /// Activation type
    pub activation_type: ActivationType,

    /// ISO reserved buffer, currently left empty
    pub buffer: [u8; 4],
}

impl DoipPayload for RoutingActivationRequest {
    fn payload_type(&self) -> PayloadType {
        PayloadType::RoutingActivationRequest
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.source_address);
        bytes.extend_from_slice(&[self.activation_type as u8]);
        bytes.extend_from_slice(&self.buffer);

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, PayloadError> {
        // Check that bytes have sufficient length
        let min_length = DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN
            + DOIP_ROUTING_ACTIVATION_REQ_TYPE_LEN_V2
            + DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN;

        if bytes.len() < min_length {
            return Err(PayloadError::RoutingActivationRequestError(
                RoutingActivationRequestError::InvalidLength,
            ));
        }

        let source_address_offset = DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN;
        let source_address: [u8; DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN] =
            match bytes[0..source_address_offset].try_into() {
                Ok(arr) => arr,
                Err(_) => {
                    return Err(PayloadError::RoutingActivationRequestError(
                        RoutingActivationRequestError::InvalidIndexRange,
                    ))
                }
            };

        let activation_type_offset = source_address_offset;

        let activation_type = match &bytes[activation_type_offset] {
            0x00 => ActivationType::Default,
            0x01 => ActivationType::WwhObd,
            0x02 => ActivationType::CentralSecurity,
            _ => {
                return Err(PayloadError::RoutingActivationRequestError(
                    RoutingActivationRequestError::InvalidActivationType,
                ))
            }
        };

        let buffer_offset = activation_type_offset + DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN;
        let buffer: [u8; DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN] =
            match bytes[activation_type_offset..buffer_offset].try_into() {
                Ok(arr) => arr,
                Err(_) => {
                    return Err(PayloadError::RoutingActivationRequestError(
                        RoutingActivationRequestError::InvalidIndexRange,
                    ))
                }
            };

        Ok(Self {
            source_address,
            activation_type,
            buffer,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        doip_message::routing_activation_request::RoutingActivationRequest,
        error::{PayloadError, RoutingActivationRequestError},
        header::{DoipPayload, PayloadType},
        message::ActivationType,
    };

    const DEFAULT_SOURCE_ADDRESS: [u8; 2] = [0x01, 0x02];
    const DEFAULT_ACTIVATION_TYPE: ActivationType = ActivationType::Default;
    const DEFAULT_BUFFER: [u8; 4] = [0x00, 0x00, 0x00, 0x00];

    #[test]
    fn test_payload_type() {
        let request = RoutingActivationRequest {
            source_address: DEFAULT_SOURCE_ADDRESS,
            activation_type: DEFAULT_ACTIVATION_TYPE,
            buffer: DEFAULT_BUFFER,
        };
        assert_eq!(
            request.payload_type(),
            PayloadType::RoutingActivationRequest
        );
    }

    #[test]
    fn test_to_bytes() {
        let request = RoutingActivationRequest {
            source_address: DEFAULT_SOURCE_ADDRESS,
            activation_type: DEFAULT_ACTIVATION_TYPE,
            buffer: DEFAULT_BUFFER,
        };
        assert_eq!(
            request.to_bytes(),
            vec![0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
    }

    #[test]
    fn test_from_bytes_too_short() {
        let request = vec![0x01, 0x02, 0x03];
        let from_bytes = RoutingActivationRequest::from_bytes(&request);

        assert!(
            from_bytes.is_err(),
            "Expected to receive an RoutingActivationRequestError::InvalidLength."
        );

        let error = from_bytes.unwrap_err();

        assert_eq!(
            error,
            PayloadError::RoutingActivationRequestError(
                RoutingActivationRequestError::InvalidLength
            ),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_invalid_activation_type() {
        let request = vec![0x01, 0x02, 0x07, 0x01, 0x02, 0x03, 0x04];
        let from_bytes = RoutingActivationRequest::from_bytes(&request);

        assert!(
            from_bytes.is_err(),
            "Expected to receive an RoutingActivationRequestError::InvalidActivationType."
        );

        let error = from_bytes.unwrap_err();

        assert_eq!(
            error,
            PayloadError::RoutingActivationRequestError(
                RoutingActivationRequestError::InvalidActivationType
            ),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_ok() {
        let request = RoutingActivationRequest {
            source_address: DEFAULT_SOURCE_ADDRESS,
            activation_type: DEFAULT_ACTIVATION_TYPE,
            buffer: DEFAULT_BUFFER,
        }
        .to_bytes();
        let from_bytes = RoutingActivationRequest::from_bytes(&request);

        assert!(
            from_bytes.is_ok(),
            "Expected RoutingActivationRequest, recieved an Error."
        );
    }
}
