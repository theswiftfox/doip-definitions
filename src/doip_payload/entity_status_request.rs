/// Requests the status of a `DoIP` Entity.
#[derive(Copy, Clone, Debug)]
pub struct EntityStatusRequest {}

impl From<EntityStatusRequest> for [u8; 0] {
    fn from(entity_status_request: EntityStatusRequest) -> Self {
        let _ = entity_status_request;
        []
    }
}

impl From<[u8; 0]> for EntityStatusRequest {
    fn from(value: [u8; 0]) -> Self {
        match value {
            [] => EntityStatusRequest {},
        }
    }
}
