use payload_type::PayloadType;
use version::ProtocolVersion;

use crate::definitions::{
    DOIP_HEADER_LEN, DOIP_INV_VERSION_LEN, DOIP_LENGTH_LEN, DOIP_TYPE_LEN, DOIP_VERSION_LEN,
};

/// The definitive fields of a `DoIP` frame.
///
/// The definition of a `DoIP` frame is found in the `DoipHeader`, this contains each
/// key field which a parser uses to identify the bytes which pertain to a `DoIP`
/// frame.
#[derive(Debug, PartialEq, Clone)]
pub struct DoipHeader {
    /// `protocol_version` acts a pair with the `inverse_protocol_version` to create
    /// a validation check to ensure the packet is constructed correctly. There
    /// are specific versions available within the `DoipVersion` enum.
    pub protocol_version: ProtocolVersion,

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

pub mod payload_type;
pub mod version;

impl From<DoipHeader> for [u8; DOIP_HEADER_LEN] {
    fn from(header: DoipHeader) -> Self {
        let protocol_version = [u8::from(header.protocol_version)];
        let inverse_protocol_version = [header.inverse_protocol_version];
        let payload_type = <[u8; DOIP_TYPE_LEN]>::from(header.payload_type);
        let payload_length = header.payload_length.to_be_bytes();

        let mut buffer = [0; DOIP_HEADER_LEN];
        let mut offset = 0;

        buffer[offset..=offset].copy_from_slice(&protocol_version);
        offset += DOIP_VERSION_LEN;

        buffer[offset..=offset].copy_from_slice(&inverse_protocol_version);
        offset += DOIP_INV_VERSION_LEN;

        buffer[offset..offset + DOIP_TYPE_LEN].copy_from_slice(&payload_type);
        offset += DOIP_TYPE_LEN;

        buffer[offset..offset + DOIP_LENGTH_LEN].copy_from_slice(&payload_length);

        buffer
    }
}

impl TryFrom<[u8; DOIP_HEADER_LEN]> for DoipHeader {
    type Error = &'static str;

    fn try_from(value: [u8; DOIP_HEADER_LEN]) -> Result<Self, Self::Error> {
        let (protocol_version_slice, rest) = value.split_at(DOIP_VERSION_LEN);
        let (inverse_protocol_version_slice, rest) = rest.split_at(DOIP_INV_VERSION_LEN);
        let (payload_type_slice, rest) = rest.split_at(DOIP_TYPE_LEN);
        let (payload_length_slice, _) = rest.split_at(DOIP_LENGTH_LEN);

        let protocol_version = ProtocolVersion::try_from(protocol_version_slice[0])?;

        let inverse_protocol_version =
            if protocol_version.validate_inverse_byte(&inverse_protocol_version_slice[0]) {
                inverse_protocol_version_slice[0]
            } else {
                return Err("Inverse protocol version validation failed.");
            };

        let mut payload_type_bytes = [0u8; DOIP_TYPE_LEN];
        payload_type_bytes.copy_from_slice(payload_type_slice);

        let payload_type = PayloadType::try_from(payload_type_bytes)?;

        let mut payload_length_bytes = [0u8; DOIP_LENGTH_LEN];
        payload_length_bytes.copy_from_slice(payload_length_slice);

        let payload_length = u32::from_be_bytes(payload_length_bytes);

        Ok(DoipHeader {
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
        definitions::DOIP_HEADER_LEN,
        header::{PayloadType, ProtocolVersion},
    };

    use super::DoipHeader;

    #[test]
    fn test_try_from_bytes() {
        let bytes: [u8; DOIP_HEADER_LEN] = [0x02, 0xfd, 0x00, 0x00, 0x01, 0x23, 0x45, 0x67];

        let header = DoipHeader::try_from(bytes);

        assert_eq!(
            header.unwrap(),
            DoipHeader {
                protocol_version: ProtocolVersion::Iso13400_2012,
                inverse_protocol_version: 0xfd,
                payload_type: PayloadType::GenericNack,
                payload_length: 0x01234567,
            }
        );
    }

    #[test]
    fn test_try_from_bytes_fail_validation() {
        let bytes: [u8; DOIP_HEADER_LEN] = [0x02, 0xff, 0x00, 0x00, 0x01, 0x23, 0x45, 0x67];

        let header = DoipHeader::try_from(bytes);

        assert_eq!(
            header.unwrap_err(),
            "Inverse protocol version validation failed."
        );
    }

    #[test]
    fn test_try_from_bytes_fail_payload_type() {
        let bytes: [u8; DOIP_HEADER_LEN] = [0x02, 0xfd, 0x90, 0x00, 0x01, 0x23, 0x45, 0x67];

        let header = DoipHeader::try_from(bytes);

        assert_eq!(header.unwrap_err(), "Invalid PayloadType.");
    }

    #[test]
    fn test_try_from_bytes_fail_protocol_version() {
        let bytes: [u8; DOIP_HEADER_LEN] = [0x09, 0xfd, 0x00, 0x00, 0x01, 0x23, 0x45, 0x67];

        let header = DoipHeader::try_from(bytes);

        assert_eq!(header.unwrap_err(), "Invalid ProtocolVersion.");
    }

    #[test]
    fn test_from_header() {
        let header = DoipHeader {
            protocol_version: ProtocolVersion::Iso13400_2012,
            inverse_protocol_version: 0xfd,
            payload_type: PayloadType::GenericNack,
            payload_length: 0x01234567,
        };

        let bytes = <[u8; DOIP_HEADER_LEN]>::from(header);

        assert_eq!(bytes, [0x02, 0xfd, 0x00, 0x00, 0x01, 0x23, 0x45, 0x67])
    }
}
