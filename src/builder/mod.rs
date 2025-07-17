use crate::doip_payload::SizedDoipPayload;
use crate::header::{PayloadType, ProtocolVersion};
use crate::payload::AliveCheckRequest;
use crate::{header::DoipHeader, message::DoipMessage, payload::DoipPayload};

/// A builder for constructing `DoipMessage` instances with specified headers and payloads.
///
/// This struct provides a fluent interface to configure the protocol version,
/// payload, and automatically populate corresponding header fields based on the payload.
#[derive(Default, Debug)]
pub struct DoipMessageBuilder {
    /// The header portion of the `DoIP` message, containing metadata like protocol version and payload type.
    header: DoipHeader,

    /// The payload content of the `DoIP` message, which varies depending on the message type.
    payload: DoipPayload,
}

impl Default for DoipPayload {
    /// Provides a default payload of type `AliveCheckRequest`.
    fn default() -> Self {
        DoipPayload::AliveCheckRequest(AliveCheckRequest {})
    }
}

impl Default for DoipHeader {
    /// Constructs a default `DoipHeader` with:
    /// - Protocol version set to `DefaultValue`
    /// - Inverse protocol version calculated automatically
    /// - Payload type set to `AliveCheckRequest`
    /// - Payload length initialized to 0
    fn default() -> Self {
        Self {
            protocol_version: ProtocolVersion::DefaultValue,
            inverse_protocol_version: !(ProtocolVersion::DefaultValue as u8),
            payload_type: PayloadType::AliveCheckRequest,
            payload_length: Default::default(),
        }
    }
}

impl DoipMessageBuilder {
    /// Creates a new `DoipMessageBuilder` instance using the default header and payload.
    ///
    /// # Example
    /// ```
    /// use doip_definitions::builder::DoipMessageBuilder;
    /// let builder = DoipMessageBuilder::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        DoipMessageBuilder::default()
    }

    /// Sets the protocol version in the header and updates the inverse protocol version accordingly.
    ///
    /// # Arguments
    ///
    /// * `protocol_version` - An object that can be converted into a `ProtocolVersion`.
    ///
    /// # Returns
    ///
    /// The updated builder instance for chaining.
    ///
    /// # Example
    /// ```
    /// use doip_definitions::builder::DoipMessageBuilder;
    /// use doip_definitions::header::ProtocolVersion;
    /// let builder = DoipMessageBuilder::new().protocol_version(ProtocolVersion::Iso13400_2012);
    /// ```
    #[must_use]
    pub fn protocol_version(mut self, protocol_version: impl Into<ProtocolVersion>) -> Self {
        self.header.protocol_version = protocol_version.into();
        self.header.inverse_protocol_version = !(self.header.protocol_version as u8);
        self
    }

    /// Sets the payload of the message and updates the header's payload type and length accordingly.
    ///
    /// # Arguments
    ///
    /// * `payload` - An object that can be converted into a `DoipPayload`.
    ///
    /// # Returns
    ///
    /// The updated builder instance for chaining.
    ///
    /// # Panics
    ///
    /// This method panics if the size of the provided payload cannot be determined using
    /// [`core::mem::size_of_val`]—which may occur if dynamically sized types or trait objects
    /// are passed in as payload components.
    ///
    /// # Example
    /// ```
    /// use doip_definitions::builder::DoipMessageBuilder;
    /// use doip_definitions::header::PayloadType;
    /// use doip_definitions::payload::AliveCheckRequest;
    /// use doip_definitions::payload::DoipPayload;
    ///
    /// let builder = DoipMessageBuilder::new().payload(DoipPayload::AliveCheckRequest(AliveCheckRequest {}));
    /// ```
    #[must_use]
    pub fn payload(mut self, payload: impl Into<DoipPayload>) -> Self {
        self.payload = payload.into();

        let payload_type = match self.payload {
            DoipPayload::GenericNack(ref _pay) => PayloadType::GenericNack,
            DoipPayload::VehicleIdentificationRequest(ref _pay) => {
                PayloadType::VehicleIdentificationRequest
            }
            DoipPayload::VehicleIdentificationRequestEid(ref _pay) => {
                PayloadType::VehicleIdentificationRequestEid
            }
            DoipPayload::VehicleIdentificationRequestVin(ref _pay) => {
                PayloadType::VehicleIdentificationRequestVin
            }
            DoipPayload::VehicleAnnouncementMessage(ref _pay) => {
                PayloadType::VehicleAnnouncementMessage
            }
            DoipPayload::RoutingActivationRequest(ref _pay) => {
                PayloadType::RoutingActivationRequest
            }
            DoipPayload::RoutingActivationResponse(ref _pay) => {
                PayloadType::RoutingActivationResponse
            }
            DoipPayload::AliveCheckRequest(ref _pay) => PayloadType::AliveCheckRequest,
            DoipPayload::AliveCheckResponse(ref _pay) => PayloadType::AliveCheckResponse,
            DoipPayload::EntityStatusRequest(ref _pay) => PayloadType::EntityStatusRequest,
            DoipPayload::EntityStatusResponse(ref _pay) => PayloadType::EntityStatusResponse,
            DoipPayload::PowerInformationRequest(ref _pay) => PayloadType::PowerInformationRequest,
            DoipPayload::PowerInformationResponse(ref _pay) => {
                PayloadType::PowerInformationResponse
            }
            DoipPayload::DiagnosticMessage(ref _pay) => PayloadType::DiagnosticMessage,
            DoipPayload::DiagnosticMessageAck(ref _pay) => PayloadType::DiagnosticMessageAck,
            DoipPayload::DiagnosticMessageNack(ref _pay) => PayloadType::DiagnosticMessageNack,
        };

        let size = self.payload.size_of();

        self.header.payload_type = payload_type;
        self.header.payload_length =
            u32::try_from(size).expect("This should never be larger than u32.");

        self
    }

    /// Finalizes the builder and returns the constructed `DoipMessage`.
    ///
    /// # Returns
    ///
    /// The fully constructed `DoipMessage`.
    ///
    /// # Example
    /// ```
    /// use doip_definitions::builder::DoipMessageBuilder;
    /// use doip_definitions::header::PayloadType;
    /// use doip_definitions::payload::AliveCheckRequest;
    /// use doip_definitions::payload::DoipPayload;
    ///
    /// let message = DoipMessageBuilder::new()
    ///     .payload(DoipPayload::AliveCheckRequest(AliveCheckRequest {}))
    ///     .build();
    /// ```
    #[must_use]
    pub fn build(self) -> DoipMessage {
        DoipMessage {
            header: self.header,
            payload: self.payload,
        }
    }
}

impl SizedDoipPayload for DoipPayload {
    /// Returns the size of the `DoipPayload` in bytes.
    fn size_of(&self) -> usize {
        match self {
            DoipPayload::GenericNack(payload) => payload.size_of(),
            DoipPayload::VehicleIdentificationRequest(payload) => payload.size_of(),
            DoipPayload::VehicleIdentificationRequestEid(payload) => payload.size_of(),
            DoipPayload::VehicleIdentificationRequestVin(payload) => payload.size_of(),
            DoipPayload::VehicleAnnouncementMessage(payload) => payload.size_of(),
            DoipPayload::RoutingActivationRequest(payload) => payload.size_of(),
            DoipPayload::RoutingActivationResponse(payload) => payload.size_of(),
            DoipPayload::AliveCheckRequest(payload) => payload.size_of(),
            DoipPayload::AliveCheckResponse(payload) => payload.size_of(),
            DoipPayload::EntityStatusRequest(payload) => payload.size_of(),
            DoipPayload::EntityStatusResponse(payload) => payload.size_of(),
            DoipPayload::PowerInformationRequest(payload) => payload.size_of(),
            DoipPayload::PowerInformationResponse(payload) => payload.size_of(),
            DoipPayload::DiagnosticMessage(payload) => payload.size_of(),
            DoipPayload::DiagnosticMessageAck(payload) => payload.size_of(),
            DoipPayload::DiagnosticMessageNack(payload) => payload.size_of(),
        }
    }
}
