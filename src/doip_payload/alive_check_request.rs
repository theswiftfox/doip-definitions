/// Checks the TCP Socket is still alive
///
/// Sent with no payload, the `AliveCheckRequest` is utilised to maintain a connection
/// to a TCP socket or to check the status of one.
#[derive(Copy, Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::AliveCheckRequest;

    #[test]
    fn test_from_bytes() {
        let n = [];
        let alive_check_req = AliveCheckRequest::from(n);
        assert_eq!(alive_check_req, AliveCheckRequest {})
    }
    #[test]
    fn test_from_alive_check_req() {
        let u = <[u8; 0]>::from(AliveCheckRequest {});
        assert_eq!(u, []);
    }
}
