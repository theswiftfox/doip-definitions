use crate::payload::{DiagnosticMessage, DoipPayload};

pub fn handle_diagnostic_message<'a>(slice: &'a [u8]) -> Result<DoipPayload<'a>, &'static str> {
    match DiagnosticMessage::try_from(slice) {
        Ok(msg) => Ok(DoipPayload::DiagnosticMessage(msg)),
        Err(_) => Err("Invalid DiagnosticMessage length."),
    }
}

#[cfg(test)]
mod tests {
    use crate::{payload::DiagnosticMessage, utils::diagnostic_message::handle_diagnostic_message};

    #[test]
    fn test_handle_diagnostic_message_pass() {
        const N: usize = 5;
        let payload = <[u8; N]>::try_from(DiagnosticMessage {
            source_address: [0x01, 0x02],
            target_address: [0x01, 0x02],
            message: &[0x05],
        })
        .unwrap();

        let result = handle_diagnostic_message(&payload);

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_diagnostic_message_invalid_length() {
        let payload = &[];
        let res = handle_diagnostic_message(payload);

        assert_eq!(res.unwrap_err(), "Invalid DiagnosticMessage length.")
    }
}
