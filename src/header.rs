use payload::{DoipPayload, PayloadType};
use version::DoipVersion;
mod payload_proto;

#[derive(Debug, PartialEq)]
pub struct DoipHeader {
    pub protocol_version: DoipVersion,
    pub inverse_protocol_version: u8,
    pub payload_type: PayloadType,
    pub payload_length: u32,
}

impl DoipHeader {
    pub fn new(protocol_version: DoipVersion, payload: &dyn DoipPayload) -> Self {
        Self {
            protocol_version,
            inverse_protocol_version: !protocol_version.to_u8(),
            payload_type: payload.payload_type(),
            payload_length: payload.to_bytes().len() as u32,
        }
    }

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
    use crate::header::{
        payload::VehicleIdentificationRequest, version::DoipVersion, DoipHeader,
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

pub mod payload {
    pub use super::payload_proto::alive_check_request::*;
    pub use super::payload_proto::alive_check_response::*;
    pub use super::payload_proto::diagnostic_message::*;
    pub use super::payload_proto::diagnostic_message_ack::*;
    pub use super::payload_proto::diagnostic_message_nack::*;
    pub use super::payload_proto::doip_payload::*;
    pub use super::payload_proto::entity_status_request::*;
    pub use super::payload_proto::entity_status_response::*;
    pub use super::payload_proto::generic_nack::*;
    pub use super::payload_proto::power_information_request::*;
    pub use super::payload_proto::power_information_response::*;
    pub use super::payload_proto::routing_activation_request::*;
    pub use super::payload_proto::routing_activation_response::*;
    pub use super::payload_proto::vehicle_announcement_message::*;
    pub use super::payload_proto::vehicle_identification_request::*;
    pub use super::payload_proto::vehicle_identification_request_eid::*;
    pub use super::payload_proto::vehicle_identification_request_vin::*;
}
pub mod version;

#[cfg(test)]
mod size_tests {
    use std::mem;

    use crate::header::{version::DoipVersion, DoipHeader};

    #[test]
    fn test_struct_sizes() {
        dbg!(mem::size_of::<DoipHeader>());
        dbg!(mem::size_of::<DoipVersion>());
    }
}
