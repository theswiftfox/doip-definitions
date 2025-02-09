use crate::{
    definitions::{
        DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN, DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN,
        DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN,
    },
    header::{DoipPayload, PayloadType},
    payload::NodeType,
};

/// Expected reponse from `EntityStatusRequest`.
///
/// Containing details of the target of the `EntityStatusRequest`, the
/// `EntityStatusResponse` provides the program with details pertaining to the
/// active status of the entity.
#[derive(Copy, Clone, Debug)]
pub struct EntityStatusResponse {
    /// The type of entity, either a `Gateway` or `Node`
    pub node_type: NodeType,

    /// The number of maximum concurrent TCP sockets allowed on this entity
    pub max_concurrent_sockets: [u8; DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN],

    /// The number of currently open TCP sockets on the entity
    pub currently_open_sockets: [u8; DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN],

    /// The max data size allowed to be sent to the entity
    pub max_data_size: [u8; DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN],
}

impl DoipPayload for EntityStatusResponse {
    fn payload_type(&self) -> PayloadType {
        PayloadType::EntityStatusResponse
    }
}
