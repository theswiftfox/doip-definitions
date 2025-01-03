use payload_type::{DoipPayload, PayloadType};
use version::DoipVersion;

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
        Self {
            protocol_version,
            inverse_protocol_version: !protocol_version.to_u8(),
            payload_type: payload.payload_type(),
            payload_length: payload.to_bytes().len() as u32,
        }
    }

    /// Converts the `DoipHeader` to bytes for ease of parsing.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        let protocol_version_bytes: Vec<u8> = [self.protocol_version.to_u8()].to_vec();
        let inverse_protocol_version_bytes: Vec<u8> = [self.inverse_protocol_version].to_vec();
        let payload_type_bytes: Vec<u8> = self.payload_type.to_bytes();
        let payload_length_bytes: Vec<u8> = self.payload_length.to_be_bytes().to_vec();

        bytes.extend_from_slice(&protocol_version_bytes);
        bytes.extend_from_slice(&inverse_protocol_version_bytes);
        bytes.extend_from_slice(&payload_type_bytes);
        bytes.extend_from_slice(&payload_length_bytes);

        bytes
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
        let payload = Box::new(VehicleIdentificationRequest {});
        let payload_ref = &*payload;
        let header = DoipHeader::new(DoipVersion::Iso13400_2012, payload_ref);

        assert_eq!(
            header.to_bytes(),
            vec![0x02, 0xfd, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00]
        );
    }
}

pub mod payload_type;
pub mod version;

#[cfg(test)]
mod size_tests {
    use std::mem;

    use crate::header::{DoipHeader, DoipVersion};

    #[test]
    fn test_struct_sizes() {
        dbg!(mem::size_of::<DoipHeader>());
        dbg!(mem::size_of::<DoipVersion>());
    }
}
