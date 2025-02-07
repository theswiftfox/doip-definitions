use payload_type::{DoipPayload, PayloadType};
use version::DoipVersion;

use crate::error::PayloadError;

/// The definitive fields of a DoIP frame.
///
/// The definition of a DoIP frame is found in the `DoipHeader`, this contains each
/// key field which a parser uses to identify the bytes which pertain to a DoIP
/// frame.
#[derive(Debug, PartialEq)]
pub struct DoipHeader {
    /// `protocol_version` acts a pair with the `inverse_protocol_version` to create
    /// a validation check to ensure the packet is constructed correctly. There
    /// are specific versions available within the `DoipVersion` enum.
    pub protocol_version: DoipVersion,

    /// Calculated using bitwise inversion, the `inverse_protocol_version` acts
    /// as a validation technique for validating the packet.
    pub inverse_protocol_version: u8,

    /// The type of payload alongside the header, this defines what is contained
    /// within the message, and directs the parser to accurately identify fields.
    ///
    /// A list of valid Payload Types are available in the `PayloadType` enum.
    pub payload_type: PayloadType,

    /// The `payload_length` is automatically calulated on the construction of a
    /// `DoipHeader` and is taken from the payload the header was initiated with.
    ///
    /// This tells the parser how many bytes to expect after the header.
    pub payload_length: u32,
}

impl DoipHeader {
    /// Construction of a new `DoipHeader` struct taking in a `DoipVersion` and
    /// a `&dyn DoipPayload`. This will then logically invert the supplied DoipVersion
    /// extract the payload type, and the calculate the payload length.
    pub fn new(protocol_version: DoipVersion, payload: &dyn DoipPayload) -> Self {
        let mut temp_buff = [0; 1024];
        let bytes = payload.to_bytes(&mut temp_buff).unwrap();
        let payload_len = bytes as u32;

        Self {
            protocol_version,
            inverse_protocol_version: !protocol_version.to_u8(),
            payload_type: payload.payload_type(),
            payload_length: payload_len,
        }
    }

    /// Converts the `DoipHeader` to bytes for ease of parsing.
    pub fn to_bytes(&self, buffer: &mut [u8]) -> Result<usize, PayloadError> {
        let mut temp_buff = [0; 1024];

        let proto_version_len = [self.protocol_version as u8].len();
        let inv_proto_version_len = [self.inverse_protocol_version as u8].len();
        let pay_type_len = self.payload_type.to_bytes(&mut temp_buff).unwrap();
        let pay_len_len = self.payload_length.to_be_bytes().len();
        let min_len = proto_version_len + inv_proto_version_len + pay_type_len + pay_len_len;

        if buffer.len() < min_len {
            return Err(PayloadError::BufferTooSmall);
        }

        let mut offset = 0;

        buffer[offset] = self.protocol_version as u8;
        offset += 1;

        buffer[offset] = self.inverse_protocol_version as u8;
        offset += 1;

        buffer[offset..offset + pay_type_len].copy_from_slice(&temp_buff[..pay_type_len]);
        offset += pay_type_len;

        buffer[offset..offset + pay_len_len].copy_from_slice(&self.payload_length.to_be_bytes());
        offset += pay_len_len;

        Ok(offset)
    }

    /// Converts a buffer source into a DoipHeader.
    pub fn from_bytes(src: &[u8]) -> Result<Self, PayloadError> {
        // Ensure that the byte slice is large enough to hold the header
        if src.len() < 8 {
            return Err(PayloadError::BufferTooSmall);
        }

        // Extract protocol_version (1 byte)
        let protocol_version = match DoipVersion::from_u8(src[0]) {
            Some(version) => version,
            None => return Err(PayloadError::InvalidProtocolVersion),
        };

        // Extract inverse_protocol_version (1 byte)
        let inverse_protocol_version = src[1];

        // Validate the inverse protocol version
        if (protocol_version as u8) + inverse_protocol_version != 0xFF {
            return Err(PayloadError::FailedProtocolCheck);
        }

        // Extract payload_type (assuming 1 byte for now, extend as needed for larger types)
        let payload_type = PayloadType::from_bytes(&src[2..3])?;

        // Extract payload_length (4 bytes)
        let payload_length = u32::from_be_bytes([src[3], src[4], src[5], src[6]]);

        // Return the constructed DoipHeader
        Ok(Self {
            protocol_version,
            inverse_protocol_version,
            payload_type,
            payload_length,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        doip_message::vehicle_identification_request::VehicleIdentificationRequest,
        header::{DoipHeader, DoipVersion},
    };

    #[test]
    fn test_to_bytes() {
        let payload = VehicleIdentificationRequest {};
        let header = DoipHeader::new(DoipVersion::Iso13400_2012, &payload);

        let mut buffer = [0; 1024];
        assert_eq!(header.to_bytes(&mut buffer), Ok(8));
    }
}

pub mod payload_type;
pub mod version;
