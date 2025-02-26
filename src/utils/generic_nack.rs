use crate::payload::{DoipPayload, GenericNack};

pub fn handle_generic_nack(slice: &[u8]) -> Result<DoipPayload<'static>, &'static str> {
    let payload_bytes: [u8; 1] = match slice.try_into() {
        Ok(bytes) => bytes,
        Err(_) => return Err("Invalid GenericNack length."),
    };

    match GenericNack::try_from(payload_bytes) {
        Ok(bytes) => Ok(DoipPayload::GenericNack(bytes)),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        payload::{DoipPayload, GenericNack, NackCode},
        utils::generic_nack::handle_generic_nack,
    };

    #[test]
    fn test_handle_generic_nack_pass() {
        let payload = &[0x00];
        let res = handle_generic_nack(payload);

        assert_eq!(
            res.unwrap(),
            DoipPayload::GenericNack(GenericNack {
                nack_code: NackCode::IncorrectPatternFormat
            })
        )
    }

    #[test]
    fn test_handle_generic_nack_invalid_length() {
        let payload = &[];
        let res = handle_generic_nack(payload);

        assert_eq!(res.unwrap_err(), "Invalid GenericNack length.")
    }

    #[test]
    fn test_handle_generic_nack_fail() {
        let payload = &[0xff];
        let res = handle_generic_nack(payload);

        assert_eq!(
            res.unwrap_err(),
            "Invalid NackCode."
        )
    }
}
