pub mod action_code;
pub mod activation_code;
pub mod activation_type;
pub mod diagnostic_ack;
pub mod diagnostic_nack;
pub mod doip_message;
pub mod nack_code;
pub mod node_type;
pub mod power_mode;
pub mod sync_status;

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

    use crate::doip_message::{
        action_code::ActionCode, activation_code::ActivationCode, activation_type::ActivationType,
        alive_check_request::AliveCheckRequest, alive_check_response::AliveCheckResponse,
        diagnostic_ack::DiagnosticAckCode, diagnostic_message::DiagnosticMessage,
        diagnostic_message_ack::DiagnosticMessageAck,
        diagnostic_message_nack::DiagnosticMessageNack, diagnostic_nack::DiagnosticNackCode,
        doip_message::DoipMessage, entity_status_request::EntityStatusRequest,
        entity_status_response::EntityStatusResponse, generic_nack::GenericNack,
        nack_code::NackCode, node_type::NodeType,
        power_information_request::PowerInformationRequest,
        power_information_response::PowerInformationResponse, power_mode::PowerMode,
        routing_activation_request::RoutingActivationRequest,
        routing_activation_response::RoutingActivationResponse, sync_status::SyncStatus,
        vehicle_announcement_message::VehicleAnnouncementMessage,
        vehicle_identification_request::VehicleIdentificationRequest,
        vehicle_identification_request_eid::VehicleIdentificationRequestEid,
        vehicle_identification_request_vin::VehicleIdentificationRequestVin,
    };

    #[test]
    fn test_struct_sizes() {
        dbg!(mem::size_of::<ActionCode>());
        dbg!(mem::size_of::<ActivationCode>());
        dbg!(mem::size_of::<ActivationType>());
        dbg!(mem::size_of::<DiagnosticAckCode>());
        dbg!(mem::size_of::<DiagnosticNackCode>());
        dbg!(mem::size_of::<DoipMessage>());
        dbg!(mem::size_of::<NackCode>());
        dbg!(mem::size_of::<NodeType>());
        dbg!(mem::size_of::<PowerMode>());
        dbg!(mem::size_of::<SyncStatus>());

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
