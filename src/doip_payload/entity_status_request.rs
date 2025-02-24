/// Requests the status of a `DoIP` Entity.
#[derive(Copy, Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::EntityStatusRequest;

    #[test]
    fn test_from_bytes() {
        let n = [];
        let entity_status_request = EntityStatusRequest::from(n);
        assert_eq!(entity_status_request, EntityStatusRequest {})
    }
    #[test]
    fn test_from_entity_status_req() {
        let u = <[u8; 0]>::from(EntityStatusRequest {});
        assert_eq!(u, []);
    }
}
