use crate::{
    definitions::{
        DOIP_COMMON_EID_LEN, DOIP_COMMON_VIN_LEN, DOIP_DIAG_COMMON_SOURCE_LEN,
        DOIP_VEHICLE_ANNOUNCEMENT_ACTION_LEN, DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN,
    },
    error::{PayloadError, VehicleAnnouncementMessageError},
    header::{DoipPayload, PayloadType},
    message::{ActionCode, SyncStatus},
};

/// Announcement from a DoIP entity in response to a
/// `VehicleIdentificationRequest[EID/VIN]`.
///
/// The positive response to a `VehicleIdentificationRequest[EID/VIN]` request
/// containing the vehicle information from the DoIP entity.
#[derive(Copy, Clone, Debug)]
pub struct VehicleAnnouncementMessage {
    /// Vehicle Identification Number
    pub vin: [u8; DOIP_COMMON_VIN_LEN],

    /// Logical address of responding entity
    pub logical_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN],

    /// Entity Identification
    pub eid: [u8; DOIP_COMMON_EID_LEN],

    /// Group Identification
    pub gid: [u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],

    /// Further actions required
    pub further_action: ActionCode,

    /// Status of VIN/GID Synchronisation
    pub vin_gid_sync: Option<SyncStatus>,
}

impl DoipPayload<'_> for VehicleAnnouncementMessage {
    fn payload_type(&self) -> PayloadType {
        PayloadType::VehicleAnnouncementMessage
    }

    fn to_bytes(&self, buffer: &mut [u8]) -> Result<usize, PayloadError> {
        let vin_len = self.vin.len();
        let log_len = self.logical_address.len();
        let eid_len = self.eid.len();
        let gid_len = self.gid.len();
        let fa_len = [self.further_action as u8].len();
        let min_len = vin_len + log_len + eid_len + gid_len + fa_len;

        if buffer.len() < min_len {
            return Err(PayloadError::BufferTooSmall);
        }

        let mut offset = 0;

        buffer[offset..offset + vin_len].copy_from_slice(&self.vin);
        offset += vin_len;

        buffer[offset..offset + log_len].copy_from_slice(&self.logical_address);
        offset += log_len;

        buffer[offset..offset + eid_len].copy_from_slice(&self.eid);
        offset += eid_len;

        buffer[offset..offset + gid_len].copy_from_slice(&self.gid);
        offset += gid_len;

        buffer[offset] = self.further_action as u8;
        offset += 1;

        Ok(offset)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, PayloadError> {
        // Check that bytes have sufficient length
        let min_length = DOIP_COMMON_VIN_LEN
            + DOIP_DIAG_COMMON_SOURCE_LEN
            + DOIP_COMMON_EID_LEN
            + DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN
            + DOIP_VEHICLE_ANNOUNCEMENT_ACTION_LEN;

        if bytes.len() < min_length {
            return Err(PayloadError::VehicleAnnouncementMessageError(
                VehicleAnnouncementMessageError::InvalidLength,
            ));
        }

        let vin_offset = DOIP_COMMON_VIN_LEN;
        let vin: [u8; DOIP_COMMON_VIN_LEN] = match bytes[0..vin_offset].try_into() {
            Ok(arr) => arr,
            Err(_) => {
                return Err(PayloadError::VehicleAnnouncementMessageError(
                    VehicleAnnouncementMessageError::InvalidIndexRange,
                ))
            }
        };

        let logical_address_offset = vin_offset + DOIP_DIAG_COMMON_SOURCE_LEN;
        let logical_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN] =
            match bytes[vin_offset..logical_address_offset].try_into() {
                Ok(arr) => arr,
                Err(_) => {
                    return Err(PayloadError::VehicleAnnouncementMessageError(
                        VehicleAnnouncementMessageError::InvalidIndexRange,
                    ))
                }
            };

        let eid_offset = logical_address_offset + DOIP_COMMON_EID_LEN;
        let eid: [u8; DOIP_COMMON_EID_LEN] =
            match bytes[logical_address_offset..eid_offset].try_into() {
                Ok(arr) => arr,
                Err(_) => {
                    return Err(PayloadError::VehicleAnnouncementMessageError(
                        VehicleAnnouncementMessageError::InvalidIndexRange,
                    ))
                }
            };

        let gid_offset = eid_offset + DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN;
        let gid: [u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN] =
            match bytes[eid_offset..gid_offset].try_into() {
                Ok(arr) => arr,
                Err(_) => {
                    return Err(PayloadError::VehicleAnnouncementMessageError(
                        VehicleAnnouncementMessageError::InvalidIndexRange,
                    ))
                }
            };

        let further_action_offset = gid_offset;
        let vin_gid_sync_offset = further_action_offset + DOIP_VEHICLE_ANNOUNCEMENT_ACTION_LEN;

        let further_action = match &bytes[further_action_offset] {
            0x00 => ActionCode::NoFurtherActionRequired,
            0x01 => ActionCode::ReservedByIso13400_01,
            0x02 => ActionCode::ReservedByIso13400_02,
            0x03 => ActionCode::ReservedByIso13400_03,
            0x04 => ActionCode::ReservedByIso13400_04,
            0x05 => ActionCode::ReservedByIso13400_05,
            0x06 => ActionCode::ReservedByIso13400_06,
            0x07 => ActionCode::ReservedByIso13400_07,
            0x08 => ActionCode::ReservedByIso13400_08,
            0x09 => ActionCode::ReservedByIso13400_09,
            0x0A => ActionCode::ReservedByIso13400_0A,
            0x0B => ActionCode::ReservedByIso13400_0B,
            0x0C => ActionCode::ReservedByIso13400_0C,
            0x0D => ActionCode::ReservedByIso13400_0D,
            0x0E => ActionCode::ReservedByIso13400_0E,
            0x0F => ActionCode::ReservedByIso13400_0F,
            0x10 => ActionCode::RoutingActivationRequired,
            _ => {
                return Err(PayloadError::VehicleAnnouncementMessageError(
                    VehicleAnnouncementMessageError::InvalidActionCode,
                ))
            }
        };

        let vin_gid_sync: Option<SyncStatus> = match bytes.get(vin_gid_sync_offset) {
            Some(0x00) => Some(SyncStatus::VinGidSynchronized),
            Some(0x01) => Some(SyncStatus::ReservedByIso13400_01),
            Some(0x02) => Some(SyncStatus::ReservedByIso13400_02),
            Some(0x03) => Some(SyncStatus::ReservedByIso13400_03),
            Some(0x04) => Some(SyncStatus::ReservedByIso13400_04),
            Some(0x05) => Some(SyncStatus::ReservedByIso13400_05),
            Some(0x06) => Some(SyncStatus::ReservedByIso13400_06),
            Some(0x07) => Some(SyncStatus::ReservedByIso13400_07),
            Some(0x08) => Some(SyncStatus::ReservedByIso13400_08),
            Some(0x09) => Some(SyncStatus::ReservedByIso13400_09),
            Some(0x0A) => Some(SyncStatus::ReservedByIso13400_0A),
            Some(0x0B) => Some(SyncStatus::ReservedByIso13400_0B),
            Some(0x0C) => Some(SyncStatus::ReservedByIso13400_0C),
            Some(0x0D) => Some(SyncStatus::ReservedByIso13400_0D),
            Some(0x0E) => Some(SyncStatus::ReservedByIso13400_0E),
            Some(0x0F) => Some(SyncStatus::ReservedByIso13400_0F),
            Some(0x10) => Some(SyncStatus::VinGidNotSynchronised),
            _ => None,
        };

        Ok(Self {
            vin,
            logical_address,
            eid,
            gid,
            further_action,
            vin_gid_sync,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        definitions::{
            DOIP_COMMON_EID_LEN, DOIP_COMMON_VIN_LEN, DOIP_DIAG_COMMON_SOURCE_LEN,
            DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN,
        },
        doip_message::vehicle_announcement_message::VehicleAnnouncementMessage,
        error::{PayloadError, VehicleAnnouncementMessageError},
        header::{DoipPayload, PayloadType},
        message::{ActionCode, SyncStatus},
    };

    const DEFAULT_VIN: [u8; DOIP_COMMON_VIN_LEN] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10, 0x11,
    ];
    const DEFAULT_LOGICAL_ADDRESS: [u8; DOIP_DIAG_COMMON_SOURCE_LEN] = [0x12, 0x13];
    const DEFAULT_EID: [u8; DOIP_COMMON_EID_LEN] = [0x14, 0x15, 0x16, 0x17, 0x18, 0x19];
    const DEFAULT_GID: [u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN] =
        [0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f];
    const DEFAULT_FURTHER_ACTION_CODE: ActionCode = ActionCode::NoFurtherActionRequired;
    const DEFAULT_VIN_GID_SYNC: Option<SyncStatus> = Some(SyncStatus::VinGidSynchronized);

    #[test]
    fn test_payload_type() {
        let request = VehicleAnnouncementMessage {
            vin: DEFAULT_VIN,
            logical_address: DEFAULT_LOGICAL_ADDRESS,
            eid: DEFAULT_EID,
            gid: DEFAULT_GID,
            further_action: DEFAULT_FURTHER_ACTION_CODE,
            vin_gid_sync: DEFAULT_VIN_GID_SYNC,
        };
        assert_eq!(
            request.payload_type(),
            PayloadType::VehicleAnnouncementMessage
        );
    }

    #[test]
    fn test_to_bytes() {
        let request = VehicleAnnouncementMessage {
            vin: DEFAULT_VIN,
            logical_address: DEFAULT_LOGICAL_ADDRESS,
            eid: DEFAULT_EID,
            gid: DEFAULT_GID,
            further_action: DEFAULT_FURTHER_ACTION_CODE,
            vin_gid_sync: DEFAULT_VIN_GID_SYNC,
        };
        let mut buffer = [0; 1024];
        assert_eq!(request.to_bytes(&mut buffer), Ok(33));
    }

    #[test]
    fn test_from_bytes_too_short() {
        let request = [0x01, 0x02, 0x03];
        let from_bytes = VehicleAnnouncementMessage::from_bytes(&request);

        assert!(
            from_bytes.is_err(),
            "Expected to receive an VehicleAnnouncementMessageError::InvalidLength."
        );

        let error = from_bytes.unwrap_err();

        assert_eq!(
            error,
            PayloadError::VehicleAnnouncementMessageError(
                VehicleAnnouncementMessageError::InvalidLength
            ),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_invalid_action_code() {
        let request = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
            0x1d, 0x1e, 0x1f, 0x11, 0x00,
        ];
        let from_bytes = VehicleAnnouncementMessage::from_bytes(&request);

        assert!(
            from_bytes.is_err(),
            "Expected to receive an VehicleAnnouncementMessageError::InvalidActionCode."
        );

        let error = from_bytes.unwrap_err();

        assert_eq!(
            error,
            PayloadError::VehicleAnnouncementMessageError(
                VehicleAnnouncementMessageError::InvalidActionCode
            ),
            "Unexpected error message: {}",
            error
        );
    }

    #[test]
    fn test_from_bytes_ok() {
        let mut buffer = [0; 1024];
        let request = VehicleAnnouncementMessage {
            vin: DEFAULT_VIN,
            logical_address: DEFAULT_LOGICAL_ADDRESS,
            eid: DEFAULT_EID,
            gid: DEFAULT_GID,
            further_action: DEFAULT_FURTHER_ACTION_CODE,
            vin_gid_sync: DEFAULT_VIN_GID_SYNC,
        }
        .to_bytes(&mut buffer)
        .unwrap();
        let from_bytes = VehicleAnnouncementMessage::from_bytes(&buffer[..request]);

        assert!(
            from_bytes.is_ok(),
            "Expected VehicleAnnouncementMessage, recieved an Error."
        );
    }
}
