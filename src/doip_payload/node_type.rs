/// Used in `EntityStatusResponse`, `NodeType` provides the possibilities of the
/// `node_type` field.
///
/// Used to understand the result of a `DoIP` packet.
#[derive(Clone, Copy, Debug, PartialEq)]
// Node Type
pub enum NodeType {
    /// Doip Gateway
    DoipGateway = 0x00,

    /// Doip Node
    DoipNode = 0x01,
}

impl From<NodeType> for u8 {
    fn from(node_type: NodeType) -> Self {
        node_type as u8
    }
}

impl TryFrom<u8> for NodeType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(NodeType::DoipGateway),
            0x01 => Ok(NodeType::DoipNode),
            _ => Err("Invalid node type."),
        }
    }
}
