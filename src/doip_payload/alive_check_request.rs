/// Checks the TCP Socket is still alive
///
/// Sent with no payload, the `AliveCheckRequest` is utilised to maintain a connection
/// to a TCP socket or to check the status of one.
#[derive(Copy, Clone, Debug)]
pub struct AliveCheckRequest {}

impl From<AliveCheckRequest> for [u8; 0] {
    fn from(alive_check_request: AliveCheckRequest) -> Self {
        let _ = alive_check_request;
        []
    }
}

impl From<[u8; 0]> for AliveCheckRequest {
    fn from(value: [u8; 0]) -> Self {
        match value {
            [] => AliveCheckRequest {},
        }
    }
}
