pub mod doip_payload;

pub mod alive_check_request;
pub mod alive_check_response;
pub mod diagnostic_message;
pub mod diagnostic_message_ack;
pub mod diagnostic_message_nack;
pub mod entity_status_request;
pub mod entity_status_response;
pub mod generic_nack;
pub mod power_information_request;
pub mod power_information_response;
pub mod routing_activation_request;
pub mod routing_activation_response;
pub mod vehicle_announcement_message;
pub mod vehicle_identification_request;
pub mod vehicle_identification_request_eid;
pub mod vehicle_identification_request_vin;

#[cfg(test)]
mod size_tests {
    use std::mem;

    use crate::header::payload::{
        AliveCheckRequest, AliveCheckResponse, DiagnosticMessage, DiagnosticMessageAck,
        DiagnosticMessageNack, EntityStatusRequest, EntityStatusResponse, GenericNack,
        PowerInformationRequest, PowerInformationResponse, RoutingActivationRequest,
        RoutingActivationResponse, VehicleAnnouncementMessage, VehicleIdentificationRequest,
        VehicleIdentificationRequestEid, VehicleIdentificationRequestVin,
    };

    #[test]
    fn test_struct_sizes() {
        dbg!(mem::size_of::<AliveCheckRequest>());
        dbg!(mem::size_of::<AliveCheckResponse>());
        dbg!(mem::size_of::<DiagnosticMessage>());
        dbg!(mem::size_of::<DiagnosticMessageAck>());
        dbg!(mem::size_of::<DiagnosticMessageNack>());
        dbg!(mem::size_of::<EntityStatusRequest>());
        dbg!(mem::size_of::<EntityStatusResponse>());
        dbg!(mem::size_of::<GenericNack>());
        dbg!(mem::size_of::<PowerInformationRequest>());
        dbg!(mem::size_of::<PowerInformationResponse>());
        dbg!(mem::size_of::<RoutingActivationRequest>());
        dbg!(mem::size_of::<RoutingActivationResponse>());
        dbg!(mem::size_of::<VehicleAnnouncementMessage>());
        dbg!(mem::size_of::<VehicleIdentificationRequest>());
        dbg!(mem::size_of::<VehicleIdentificationRequestEid>());
        dbg!(mem::size_of::<VehicleIdentificationRequestVin>());
    }
}
