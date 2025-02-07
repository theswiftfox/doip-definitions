use crate::{
    definitions::DOIP_COMMON_EID_LEN,
    error::{PayloadError, VehicleIdentificationRequestEidError},
    header::{DoipPayload, PayloadType},
};

/// Requests a `VehicleAnnouncementMessage` from entities with the same EID
///
/// Matches DoIP entities with the same EID for response to the request.
#[derive(Copy, Clone, Debug)]
pub struct VehicleIdentificationRequestEid {
    /// Entity Identification
    pub eid: [u8; DOIP_COMMON_EID_LEN],
}

impl DoipPayload<'_> for VehicleIdentificationRequestEid {
    fn payload_type(&self) -> PayloadType {
        PayloadType::VehicleIdentificationRequestEid
    }

    fn to_bytes(&self, buffer: &mut [u8]) -> Result<usize, PayloadError> {
        let eid_len = self.eid.len();
        let min_len = eid_len;

        if buffer.len() < min_len {
            return Err(PayloadError::BufferTooSmall);
        }

        let mut offset = 0;

        buffer[offset..offset + eid_len].copy_from_slice(&self.eid);
        offset += eid_len;

        Ok(offset)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, PayloadError> {
        // Check that bytes have sufficient length
        let min_length = DOIP_COMMON_EID_LEN;

        if bytes.len() < min_length {
            return Err(PayloadError::VehicleIdentificationRequestEidError(
                VehicleIdentificationRequestEidError::InvalidLength,
            ));
        }

        let eid_offset = DOIP_COMMON_EID_LEN;
        let eid: [u8; DOIP_COMMON_EID_LEN] = match bytes[0..eid_offset].try_into() {
            Ok(arr) => arr,
            Err(_) => {
                return Err(PayloadError::VehicleIdentificationRequestEidError(
                    VehicleIdentificationRequestEidError::InvalidIndexRange,
                ))
            }
        };

        Ok(Self { eid })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        definitions::DOIP_COMMON_EID_LEN,
        doip_message::vehicle_identification_request_eid::VehicleIdentificationRequestEid,
        error::{PayloadError, VehicleIdentificationRequestEidError},
        header::{DoipPayload, PayloadType},
    };

    const DEFAULT_EID: [u8; DOIP_COMMON_EID_LEN] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05];

    #[test]
    fn test_payload_type() {
        let request = VehicleIdentificationRequestEid { eid: DEFAULT_EID };
        assert_eq!(
            request.payload_type(),
            PayloadType::VehicleIdentificationRequestEid
        );
    }

    #[test]
    fn test_to_bytes() {
        let mut buffer = [0; 1024];
        let request = VehicleIdentificationRequestEid { eid: DEFAULT_EID };
        assert_eq!(request.to_bytes(&mut buffer), Ok(DEFAULT_EID.len()));
    }

    #[test]
    fn test_from_bytes_invalid_length() {
        let bytes = [0x00, 0x01, 0x02, 0x03, 0x04];
        let request = VehicleIdentificationRequestEid::from_bytes(&bytes);

        assert!(
            request.is_err(),
            "Expected to receive an VehicleIdentificationRequestEidParse::InvalidLength."
        );

        let error = request.unwrap_err();

        assert_eq!(
            error,
            PayloadError::VehicleIdentificationRequestEidError(
                VehicleIdentificationRequestEidError::InvalidLength
            ),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_ok() {
        let mut buffer = [0; 1024];
        let bytes = VehicleIdentificationRequestEid { eid: DEFAULT_EID }
            .to_bytes(&mut buffer)
            .unwrap();
        let request = VehicleIdentificationRequestEid::from_bytes(&buffer[..bytes]);

        assert!(
            request.is_ok(),
            "Expected VehicleIdentificationRequestEid, recieved an Error."
        );
    }
}
