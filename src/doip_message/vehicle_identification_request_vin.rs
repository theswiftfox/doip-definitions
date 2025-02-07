use crate::{
    definitions::DOIP_COMMON_VIN_LEN,
    error::{PayloadError, VehicleIdentificationRequestVinError},
    header::{DoipPayload, PayloadType},
};

/// Requests a `VehicleAnnouncementMessage` from entities with the same VIN
///
/// Matches DoIP entities with the same VIN for response to the request.
#[derive(Copy, Clone, Debug)]
pub struct VehicleIdentificationRequestVin {
    /// Vehicle Identification Number
    pub vin: [u8; DOIP_COMMON_VIN_LEN],
}

impl DoipPayload<'_> for VehicleIdentificationRequestVin {
    fn payload_type(&self) -> PayloadType {
        PayloadType::VehicleIdentificationRequestVin
    }

    fn to_bytes(&self, buffer: &mut [u8]) -> Result<usize, PayloadError> {
        let vin_len = self.vin.len();
        let min_len = vin_len;

        if buffer.len() < min_len {
            return Err(PayloadError::BufferTooSmall);
        }

        let mut offset = 0;

        buffer[offset..offset + vin_len].copy_from_slice(&self.vin);
        offset += vin_len;

        Ok(offset)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, PayloadError> {
        // Check that bytes have sufficient length
        let min_length = DOIP_COMMON_VIN_LEN;

        if bytes.len() < min_length {
            return Err(PayloadError::VehicleIdentificationRequestVinError(
                VehicleIdentificationRequestVinError::InvalidLength,
            ));
        }

        let vin_offset = DOIP_COMMON_VIN_LEN;
        let vin: [u8; DOIP_COMMON_VIN_LEN] = match bytes[0..vin_offset].try_into() {
            Ok(arr) => arr,
            Err(_) => {
                return Err(PayloadError::VehicleIdentificationRequestVinError(
                    VehicleIdentificationRequestVinError::InvalidIndexRange,
                ))
            }
        };

        Ok(Self { vin })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        definitions::DOIP_COMMON_VIN_LEN,
        doip_message::vehicle_identification_request_vin::VehicleIdentificationRequestVin,
        error::{PayloadError, VehicleIdentificationRequestVinError},
        header::{DoipPayload, PayloadType},
    };

    const DEFAULT_VIN: [u8; DOIP_COMMON_VIN_LEN] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11,
    ];

    #[test]
    fn test_payload_type() {
        let request = VehicleIdentificationRequestVin { vin: DEFAULT_VIN };
        assert_eq!(
            request.payload_type(),
            PayloadType::VehicleIdentificationRequestVin
        );
    }

    #[test]
    fn test_to_bytes() {
        let mut buffer = [0; 1024];
        let request = VehicleIdentificationRequestVin { vin: DEFAULT_VIN };
        assert_eq!(request.to_bytes(&mut buffer), Ok(DEFAULT_VIN.len()));
    }

    #[test]
    fn test_from_bytes_invalid_length() {
        let bytes = [0x00, 0x01, 0x02, 0x03, 0x04];
        let request = VehicleIdentificationRequestVin::from_bytes(&bytes);

        assert!(
            request.is_err(),
            "Expected to receive an VehicleIdentificationRequestVinParse::InvalidLength."
        );

        let error = request.unwrap_err();

        assert_eq!(
            error,
            PayloadError::VehicleIdentificationRequestVinError(
                VehicleIdentificationRequestVinError::InvalidLength
            ),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_ok() {
        let mut buffer = [0; 1024];
        let bytes = VehicleIdentificationRequestVin { vin: DEFAULT_VIN }
            .to_bytes(&mut buffer)
            .unwrap();
        let request = VehicleIdentificationRequestVin::from_bytes(&buffer[..bytes]);

        assert!(
            request.is_ok(),
            "Expected VehicleIdentificationRequestVin, recieved an Error."
        );
    }
}
