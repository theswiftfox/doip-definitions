use crate::definitions::DOIP_DIAG_COMMON_SOURCE_LEN;

/// Confirmation of the `AliveCheckRequest`.
///
/// The typical response from an `AliveCheckRequest`.

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AliveCheckResponse {
    /// The source address of the responding `DoIP` Entity
    pub source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN],
}
