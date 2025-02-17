use crate::definitions::DOIP_DIAG_COMMON_SOURCE_LEN;

/// Confirmation of the `AliveCheckRequest`.
///
/// The typical response from an `AliveCheckRequest`.
#[derive(Copy, Clone, Debug)]
pub struct AliveCheckResponse {
    /// The source address of the responding `DoIP` Entity
    pub source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN],
}

impl From<AliveCheckResponse> for [u8; DOIP_DIAG_COMMON_SOURCE_LEN] {
    fn from(alive_check_response: AliveCheckResponse) -> Self {
        alive_check_response.source_address
    }
}

impl From<[u8; DOIP_DIAG_COMMON_SOURCE_LEN]> for AliveCheckResponse {
    fn from(value: [u8; DOIP_DIAG_COMMON_SOURCE_LEN]) -> Self {
        AliveCheckResponse {
            source_address: value,
        }
    }
}
