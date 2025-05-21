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
