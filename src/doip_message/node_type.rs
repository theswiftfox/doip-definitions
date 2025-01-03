use std::fmt;

/// Used in `EntityStatusResponse`, `NodeType` provides the possibilities of the
/// `node_type` field.
///
/// Used to understand the result of a DoIP packet.
#[derive(Clone, Copy, Debug, PartialEq)]
// Node Type
pub enum NodeType {
    /// Doip Gateway
    DoipGateway = 0x00,

    /// Doip Node
    DoipNode = 0x01,
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let node_type_strings = match self {
            NodeType::DoipGateway => "DoIP gateway",
            NodeType::DoipNode => "DoIP node",
        };
        write!(f, "{}", node_type_strings)
    }
}
