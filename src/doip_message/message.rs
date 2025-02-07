use crate::{
    definitions::DOIP_HEADER_LEN,
    error::PayloadError,
    header::{DoipHeader, DoipPayload, DoipVersion},
};

/// The decoded struct of a DoIP packet.
///
/// Each DoIP packet contains a header which describes the message, this is outlined
/// in `DoipHeader`.
///
/// Some Payload Types available in DoIP require a payload which is covered by
/// `DoipPayload`.
#[derive(Debug)]
pub struct DoipMessage<P: for<'a> DoipPayload<'a>> {
    /// Defined by `DoipHeader`, the header supplies the information for programs
    /// to understand the payload.
    pub header: DoipHeader,

    /// Takes any struct implementing `DoipPayload`.
    pub payload: P,
}

impl<P> DoipMessage<P>
where
    P: for<'a> DoipPayload<'a>, // Use `for<'a>` here
{
    /// Constructs a new `DoipMessage`.
    pub fn new(protocol_version: DoipVersion, payload: P) -> Self {
        Self {
            header: DoipHeader::new(protocol_version, &payload),
            payload,
        }
    }

    /// Converts the currently `DoipMessage` to a Vec of bytes.
    pub fn to_bytes(&self, buffer: &mut [u8]) -> Result<usize, PayloadError> {
        let header_len = self.header.to_bytes(buffer).unwrap();
        let payload_len = self.payload.to_bytes(buffer).unwrap();

        Ok(header_len + payload_len)
    }

    /// Creates a `DoipMessage` from bytes.
    pub fn from_bytes<'a>(src: &'a [u8]) -> Result<DoipMessage<P>, PayloadError> {
        if src.is_empty() {
            return Err(PayloadError::EmptyInput);
        }

        // First, decode the header
        let header = DoipHeader::from_bytes(src)?;

        // Next, extract the payload size based on header information (use `DoipHeader` or `payload_type`).
        let payload_length = header.payload_length as usize;

        // Extract the payload data
        let payload_data = match src.get(DOIP_HEADER_LEN..DOIP_HEADER_LEN + payload_length) {
            Some(data) => data,
            None => return Err(PayloadError::IncompletePayload),
        };

        // Decode the payload
        let payload = P::from_bytes(payload_data)?;

        // Return the `DoipMessage` with the header and decoded payload
        Ok(DoipMessage { header, payload })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        doip_message::vehicle_identification_request::VehicleIdentificationRequest,
        error::PayloadError,
        header::{DoipPayload, DoipVersion},
    };

    use super::DoipMessage;

    #[test]
    fn test_to_bytes() {
        let mut buffer = [0; 1024];
        let msg = DoipMessage::new(DoipVersion::Iso13400_2012, VehicleIdentificationRequest {});

        let bytes = msg.to_bytes(&mut buffer);

        assert_eq!(bytes, Ok(8))
    }

    #[test]
    fn test_parse_from_bytes_ok() {
        let bytes = [0x02, 0xfd, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00];
        let msg_raw = DoipMessage::from_bytes(&bytes);

        assert!(msg_raw.is_ok(), "Expected msg to be ok.");

        let msg: DoipMessage<VehicleIdentificationRequest> = msg_raw.unwrap();

        let mut buffer = [0; 1024];
        let comp = DoipMessage::new(DoipVersion::Iso13400_2012, VehicleIdentificationRequest {});
        let msg_len_raw = (&msg.payload).to_bytes(&mut buffer);
        let comp_len_raw = (&comp.payload).to_bytes(&mut buffer);

        assert_eq!(msg.header, comp.header);
        assert_eq!(msg_len_raw, comp_len_raw);
    }

    #[test]
    fn test_parse_from_bytes_empty() {
        let bytes = [];
        let msg_raw: Result<DoipMessage<VehicleIdentificationRequest>, _> =
            DoipMessage::from_bytes(&bytes);

        assert!(
            msg_raw.is_err(),
            "Expected to receive an ParseError::EmptyInput."
        );

        let error = msg_raw.unwrap_err();

        assert_eq!(
            error,
            PayloadError::EmptyInput,
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_parse_from_bytes_invalid_protocol() {
        let bytes = [0x05, 0xfd, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00];
        let msg_raw: Result<DoipMessage<VehicleIdentificationRequest>, _> =
            DoipMessage::from_bytes(&bytes);

        assert!(
            msg_raw.is_err(),
            "Expected to receive an ParseError::InvalidProtocolVersion."
        );

        let error = msg_raw.unwrap_err();

        assert_eq!(
            error,
            PayloadError::InvalidProtocolVersion,
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_parse_from_bytes_failed_protocol_check() {
        let bytes = [0x02, 0xff, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00];
        let msg_raw: Result<DoipMessage<VehicleIdentificationRequest>, _> =
            DoipMessage::from_bytes(&bytes);

        assert!(
            msg_raw.is_err(),
            "Expected to receive an ParseError::FailedProtocolCheck."
        );

        let error = msg_raw.unwrap_err();

        assert_eq!(
            error,
            PayloadError::FailedProtocolCheck,
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_parse_from_bytes_payload_type_err() {
        let bytes = [0x02, 0xfd, 0x90, 0x01, 0x00, 0x00, 0x00, 0x00];
        let msg_raw: Result<DoipMessage<VehicleIdentificationRequest>, _> =
            DoipMessage::from_bytes(&bytes);

        assert!(
            msg_raw.is_err(),
            "Expected to receive an PayloadError::InvalidPayloadType."
        );

        let error = msg_raw.unwrap_err();

        assert_eq!(
            error,
            PayloadError::InvalidPayloadType,
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_parse_from_bytes_incomplete_data() {
        let bytes = [0x02, 0xfd, 0x40, 0x02, 0x00, 0x00, 0x00, 0x07, 0x00];
        let msg_raw: Result<DoipMessage<VehicleIdentificationRequest>, _> =
            DoipMessage::from_bytes(&bytes);

        assert!(
            msg_raw.is_err(),
            "Expected to receive an ParseError::IncompletePayload."
        );

        let error = msg_raw.unwrap_err();

        assert_eq!(
            error,
            PayloadError::IncompletePayload,
            "Unexpected error message: {}",
            error
        );
    }
}
