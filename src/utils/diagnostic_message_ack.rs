use crate::{
    definitions::{DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN},
    payload::{DiagnosticMessageAck, DoipPayload},
};

pub fn handle_diagnostic_message_ack(slice: &[u8]) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1] =
        match slice.try_into() {
            Ok(bytes) => bytes,
            Err(_) => return Err("Invalid DiagnosticMessageAck length."),
        };

    match DiagnosticMessageAck::try_from(payload_bytes) {
        Ok(bytes) => Ok(DoipPayload::DiagnosticMessageAck(bytes)),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        definitions::{DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN},
        payload::{DiagnosticAckCode, DiagnosticMessageAck, DoipPayload},
        utils::diagnostic_message_ack::handle_diagnostic_message_ack,
    };

    #[test]
    fn test_handle_diagnostic_message_ack_pass() {
        let payload = <[u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]>::from(
            DiagnosticMessageAck {
                source_address: [0x01, 0x02],
                target_address: [0x01, 0x02],
                ack_code: DiagnosticAckCode::Acknowledged,
            },
        );

        let result = handle_diagnostic_message_ack(&payload);

        assert_eq!(
            result.unwrap(),
            DoipPayload::DiagnosticMessageAck(DiagnosticMessageAck {
                source_address: [0x01, 0x02],
                target_address: [0x01, 0x02],
                ack_code: DiagnosticAckCode::Acknowledged,
            })
        );
    }

    #[test]
    fn test_handle_diagnostic_message_ack_invalid_length() {
        let payload = &[];
        let res = handle_diagnostic_message_ack(payload);

        assert_eq!(res.unwrap_err(), "Invalid DiagnosticMessageAck length.")
    }

    #[test]
    fn test_handle_diagnostic_message_ack_fail() {
        let payload: [u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1] =
            [0x01, 0x02, 0x03, 0x04, 0xff];

        let result = handle_diagnostic_message_ack(&payload);

        assert_eq!(result.unwrap_err(), "Invalid DiagnosticAckCode.")
    }
}
