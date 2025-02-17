use crate::{
    definitions::{
        DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN, DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN,
        DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN,
    },
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

impl From<EntityStatusResponse>
    for [u8; 1
        + DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN
        + DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN
        + DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN]
{
    fn from(entity_status_request: EntityStatusResponse) -> Self {
        let node_type = [u8::from(entity_status_request.node_type)];
        let max_concurrent_sockets = entity_status_request.max_concurrent_sockets;
        let currently_open_sockets = entity_status_request.currently_open_sockets;
        let max_data_size = entity_status_request.max_data_size;

        let mut buffer = [0; 1
            + DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN
            + DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN
            + DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN];
        let mut offset = 0;

        buffer[offset] = node_type[0];
        offset += 1;

        buffer[offset..offset + DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN]
            .copy_from_slice(&max_concurrent_sockets);
        offset += DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN;

        buffer[offset..offset + DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN]
            .copy_from_slice(&currently_open_sockets);
        offset += DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN;

        buffer[offset..offset + DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN]
            .copy_from_slice(&max_data_size);

        buffer
    }
}

impl
    TryFrom<
        [u8; 1
            + DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN
            + DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN
            + DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN],
    > for EntityStatusResponse
{
    type Error = &'static str;

    fn try_from(
        value: [u8; 1
            + DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN
            + DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN
            + DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN],
    ) -> Result<Self, Self::Error> {
        let (node_slice, rest) = value.split_at(1);

        let (max_concurrent_sockets_slice, rest) =
            rest.split_at(DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN);

        let (currently_open_sockets_slice, rest) =
            rest.split_at(DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN);

        let (max_data_size_slice, _) = rest.split_at(DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN);

        let node_type = NodeType::try_from(node_slice[0])?;

        let max_concurrent_sockets: [u8; DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN] =
            max_concurrent_sockets_slice
                .try_into()
                .map_err(|_| "Invalid max concurrent sockets length")?;

        let currently_open_sockets: [u8; DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN] =
            currently_open_sockets_slice
                .try_into()
                .map_err(|_| "Invalid max concurrent sockets length")?;

        let max_data_size: [u8; DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN] = max_data_size_slice
            .try_into()
            .map_err(|_| "Invalid max concurrent sockets length")?;

        Ok(EntityStatusResponse {
            node_type,
            max_concurrent_sockets,
            currently_open_sockets,
            max_data_size,
        })
    }
}
