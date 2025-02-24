use crate::definitions::DOIP_DIAG_COMMON_SOURCE_LEN;

/// Confirmation of the `AliveCheckRequest`.
///
/// The typical response from an `AliveCheckRequest`.
#[derive(Copy, Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use crate::definitions::DOIP_DIAG_COMMON_SOURCE_LEN;

    use super::AliveCheckResponse;

    #[test]
    fn test_from_bytes() {
        for n in u16::MIN..=u16::MAX {
            let bytes: [u8; DOIP_DIAG_COMMON_SOURCE_LEN] = n.to_be_bytes();
            let alive_check_res = AliveCheckResponse::from(bytes);

            assert_eq!(bytes, alive_check_res.source_address)
        }
    }

    #[test]
    fn test_from_alive_check_res() {
        for n in u16::MIN..=u16::MAX {
            let u = <[u8; 2]>::from(AliveCheckResponse {
                source_address: n.to_be_bytes(),
            });
            assert_eq!(u, n.to_be_bytes());
        }
    }
}
