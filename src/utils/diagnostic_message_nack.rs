use crate::{
    definitions::{DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN},
    payload::{DiagnosticMessageNack, DoipPayload},
};

pub fn handle_diagnostic_message_nack(slice: &[u8]) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1] =
        match slice.try_into() {
            Ok(bytes) => bytes,
            Err(_) => return Err("Invalid DiagnosticMessageNack length."),
        };

    match DiagnosticMessageNack::try_from(payload_bytes) {
        Ok(bytes) => Ok(DoipPayload::DiagnosticMessageNack(bytes)),
        Err(e) => Err(e),
    }
}
#[cfg(test)]
mod tests {
    use crate::{
        definitions::{DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN},
        payload::{DiagnosticMessageNack, DiagnosticNackCode},
        utils::diagnostic_message_nack::handle_diagnostic_message_nack,
    };

    #[test]
    fn test_handle_diagnostic_message_nack_pass() {
        let payload =
            <[u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]>::try_from(
                DiagnosticMessageNack {
                    source_address: [0x01, 0x02],
                    target_address: [0x01, 0x02],
                    nack_code: DiagnosticNackCode::OutOfMemory,
                },
            )
            .unwrap();

        let result = handle_diagnostic_message_nack(&payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_diagnostic_message_nack_invalid_length() {
        let payload = &[];
        let res = handle_diagnostic_message_nack(payload);

        assert_eq!(res.unwrap_err(), "Invalid DiagnosticMessageNack length.")
    }

    #[test]
    fn test_handle_diagnostic_message_nack_fail() {
        let payload: [u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1] =
            [0x01, 0x02, 0x03, 0x04, 0xff];

        let result = handle_diagnostic_message_nack(&payload);

        assert_eq!(result.unwrap_err(), "Invalid DiagnosticNackCode.")
    }
}
