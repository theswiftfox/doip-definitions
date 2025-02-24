use crate::{
    definitions::{
        DOIP_ENTITY_STATUS_RESPONSE_LEN, DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN,
        DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN, DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN,
        DOIP_ENTITY_STATUS_RESPONSE_NODE_LEN,
    },
    payload::NodeType,
};

/// Expected reponse from `EntityStatusRequest`.
///
/// Containing details of the target of the `EntityStatusRequest`, the
/// `EntityStatusResponse` provides the program with details pertaining to the
/// active status of the entity.
#[derive(Copy, Clone, Debug, PartialEq)]
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

impl From<EntityStatusResponse> for [u8; DOIP_ENTITY_STATUS_RESPONSE_LEN] {
    fn from(entity_status_request: EntityStatusResponse) -> Self {
        let node_type = [u8::from(entity_status_request.node_type)];
        let max_concurrent_sockets = entity_status_request.max_concurrent_sockets;
        let currently_open_sockets = entity_status_request.currently_open_sockets;
        let max_data_size = entity_status_request.max_data_size;

        let mut buffer = [0; DOIP_ENTITY_STATUS_RESPONSE_LEN];
        let mut offset = 0;

        buffer[offset] = node_type[0];
        offset += DOIP_ENTITY_STATUS_RESPONSE_NODE_LEN;

        buffer[offset..=offset].copy_from_slice(&max_concurrent_sockets);
        offset += DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN;

        buffer[offset..=offset].copy_from_slice(&currently_open_sockets);
        offset += DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN;

        buffer[offset..offset + DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN]
            .copy_from_slice(&max_data_size);

        buffer
    }
}

impl TryFrom<[u8; DOIP_ENTITY_STATUS_RESPONSE_LEN]> for EntityStatusResponse {
    type Error = &'static str;

    fn try_from(value: [u8; DOIP_ENTITY_STATUS_RESPONSE_LEN]) -> Result<Self, Self::Error> {
        let (node_slice, rest) = value.split_at(DOIP_ENTITY_STATUS_RESPONSE_NODE_LEN);

        let (max_concurrent_sockets_slice, rest) =
            rest.split_at(DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN);

        let (currently_open_sockets_slice, rest) =
            rest.split_at(DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN);

        let (max_data_size_slice, _) = rest.split_at(DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN);

        let node_type = NodeType::try_from(node_slice[0])?;

        let mut max_concurrent_sockets = [0u8; DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN];
        max_concurrent_sockets.copy_from_slice(max_concurrent_sockets_slice);

        let mut currently_open_sockets = [0u8; DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN];
        currently_open_sockets.copy_from_slice(currently_open_sockets_slice);

        let mut max_data_size = [0u8; DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN];
        max_data_size.copy_from_slice(max_data_size_slice);

        Ok(EntityStatusResponse {
            node_type,
            max_concurrent_sockets,
            currently_open_sockets,
            max_data_size,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{definitions::DOIP_ENTITY_STATUS_RESPONSE_LEN, payload::NodeType};

    use super::EntityStatusResponse;

    #[test]
    fn test_try_from_bytes() {
        for a in u8::MIN..u8::MAX {
            let ent_res = EntityStatusResponse::try_from([a, 0x12, 0x34, 0x56, 0x78, 0x90, 0x09]);

            match a {
                0x00 => assert_eq!(
                    ent_res.unwrap(),
                    EntityStatusResponse {
                        node_type: NodeType::DoipGateway,
                        max_concurrent_sockets: [0x12],
                        currently_open_sockets: [0x34],
                        max_data_size: [0x56, 0x78, 0x90, 0x09]
                    }
                ),
                0x01 => assert_eq!(
                    ent_res.unwrap(),
                    EntityStatusResponse {
                        node_type: NodeType::DoipNode,
                        max_concurrent_sockets: [0x12],
                        currently_open_sockets: [0x34],
                        max_data_size: [0x56, 0x78, 0x90, 0x09]
                    }
                ),
                _ => assert_eq!(ent_res.unwrap_err(), "Invalid NodeType."),
            };
        }
    }

    #[test]
    fn test_from_entity_status_res() {
        let ent_res = EntityStatusResponse {
            node_type: NodeType::DoipGateway,
            max_concurrent_sockets: [0x01],
            currently_open_sockets: [0x02],
            max_data_size: [0x03, 0x04, 0x05, 0x06],
        };

        let ent_res_bytes = <[u8; DOIP_ENTITY_STATUS_RESPONSE_LEN]>::from(ent_res);

        assert_eq!(ent_res_bytes, [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06])
    }
}
