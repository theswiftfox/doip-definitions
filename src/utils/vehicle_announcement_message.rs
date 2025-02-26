use crate::payload::{DoipPayload, VehicleAnnouncementMessage};

pub fn handle_vehicle_announcement_message(
    slice: &[u8],
) -> Result<DoipPayload<'static>, &'static str> {
    let bytes = VehicleAnnouncementMessage::try_from(slice)?;

    Ok(DoipPayload::VehicleAnnouncementMessage(bytes))
}

#[cfg(test)]
mod tests {
    use crate::{
        definitions::{DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG, DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT},
        utils::vehicle_announcement_message::handle_vehicle_announcement_message,
    };

    #[test]
    fn test_handle_vehicle_announcement_message_long_pass() {
        let payload = &[0u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG];
        let res = handle_vehicle_announcement_message(payload);

        assert!(res.is_ok())
    }

    #[test]
    fn test_handle_vehicle_announcement_message_long_invalid_length() {
        let payload = &[0x00];
        let res = handle_vehicle_announcement_message(payload);

        assert_eq!(res.unwrap_err(), "Invalid buffer size")
    }

    #[test]
    fn test_handle_vehicle_announcement_message_long_fail() {
        let payload: [u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG] = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
            0x1d, 0x1e, 0x1f, 0xff, 0xff,
        ];
        let res = handle_vehicle_announcement_message(&payload);

        assert_eq!(res.unwrap_err(), "Invalid ActionCode.")
    }

    #[test]
    fn test_handle_vehicle_announcement_message_short_pass() {
        let payload = &[0u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT];
        let res = handle_vehicle_announcement_message(payload);

        assert!(res.is_ok())
    }

    #[test]
    fn test_handle_vehicle_announcement_message_short_invalid_length() {
        let payload = &[0x00];
        let res = handle_vehicle_announcement_message(payload);

        assert_eq!(res.unwrap_err(), "Invalid buffer size")
    }

    #[test]
    fn test_handle_vehicle_announcement_message_short_fail() {
        let payload: [u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT] = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
            0x1d, 0x1e, 0x1f, 0xff,
        ];
        let res = handle_vehicle_announcement_message(&payload);

        assert_eq!(res.unwrap_err(), "Invalid ActionCode.")
    }
}
