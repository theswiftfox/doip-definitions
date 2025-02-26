use crate::{
    definitions::{
        DOIP_COMMON_EID_LEN, DOIP_COMMON_VIN_LEN, DOIP_DIAG_COMMON_SOURCE_LEN,
        DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN, DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG,
        DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT,
    },
    payload::{ActionCode, SyncStatus},
};

/// Announcement from a `DoIP` entity in response to a
/// `VehicleIdentificationRequest[EID/VIN]`.
///
/// The positive response to a `VehicleIdentificationRequest[EID/VIN]` request
/// containing the vehicle information from the `DoIP` entity.
#[derive(Copy, Clone, Debug, PartialEq)]
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

impl<const N: usize> TryFrom<VehicleAnnouncementMessage> for [u8; N] {
    type Error = &'static str;

    fn try_from(
        vehicle_announcement_message: VehicleAnnouncementMessage,
    ) -> Result<Self, Self::Error> {
        vehicle_announcement_message
            .vin_gid_sync
            .map_or(DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT, |_| {
                DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG
            })
            .le(&N)
            .then_some(())
            .ok_or("Invalid buffer size")?;

        let mut buffer = [0; N];
        let mut offset = 0;

        buffer[offset..offset + DOIP_COMMON_VIN_LEN]
            .copy_from_slice(&vehicle_announcement_message.vin);
        offset += DOIP_COMMON_VIN_LEN;

        buffer[offset..offset + DOIP_DIAG_COMMON_SOURCE_LEN]
            .copy_from_slice(&vehicle_announcement_message.logical_address);
        offset += DOIP_DIAG_COMMON_SOURCE_LEN;

        buffer[offset..offset + DOIP_COMMON_EID_LEN]
            .copy_from_slice(&vehicle_announcement_message.eid);
        offset += DOIP_COMMON_EID_LEN;

        buffer[offset..offset + DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN]
            .copy_from_slice(&vehicle_announcement_message.gid);
        offset += DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN;

        buffer[offset] = u8::from(vehicle_announcement_message.further_action);
        offset += 1;

        buffer
            .get_mut(offset)
            .filter(|_| N >= DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG)
            .map(|b| {
                *b = vehicle_announcement_message
                    .vin_gid_sync
                    .map_or(0, u8::from);
            });

        Ok(buffer)
    }
}

impl<'a> TryFrom<&'a [u8]> for VehicleAnnouncementMessage {
    type Error = &'static str;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let len = value.len();

        let expected_len = if len == DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT {
            DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT
        } else if len == DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG {
            DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG
        } else {
            return Err("Invalid buffer size");
        };

        let (vin_slice, rest) = value.split_at(DOIP_COMMON_VIN_LEN);
        let (logical_address_slice, rest) = rest.split_at(DOIP_DIAG_COMMON_SOURCE_LEN);
        let (eid_slice, rest) = rest.split_at(DOIP_COMMON_EID_LEN);
        let (gid_slice, rest) = rest.split_at(DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN);
        let (further_action_slice, rest) = rest.split_at(1);

        let mut vin = [0u8; DOIP_COMMON_VIN_LEN];
        vin.copy_from_slice(vin_slice);

        let mut logical_address = [0u8; DOIP_DIAG_COMMON_SOURCE_LEN];
        logical_address.copy_from_slice(logical_address_slice);

        let mut eid = [0u8; DOIP_COMMON_EID_LEN];
        eid.copy_from_slice(eid_slice);

        let mut gid = [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN];
        gid.copy_from_slice(gid_slice);

        let further_action = ActionCode::try_from(further_action_slice[0])?;

        let vin_gid_sync = if expected_len == DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG {
            let (vin_gid_sync_slice, _) = rest.split_at(1);
            Some(SyncStatus::try_from(vin_gid_sync_slice[0])?)
        } else {
            None
        };

        Ok(VehicleAnnouncementMessage {
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
            DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN, DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG,
            DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT,
        },
        payload::{ActionCode, SyncStatus},
    };

    use super::VehicleAnnouncementMessage;

    #[test]
    fn test_try_from_bytes_long() {
        for a in u16::MIN..=u16::MAX {
            let (b, c) = (a.to_be_bytes()[0], a.to_be_bytes()[1]);
            let bytes: &[u8] = &[
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
                0x1d, 0x1e, 0x1f, b, c,
            ];
            let vam = VehicleAnnouncementMessage::try_from(bytes);

            match (b, c) {
                (0x00..=0x10, 0x00..=0x10) => {
                    assert!(ActionCode::try_from(b).is_ok());
                    let further_action = ActionCode::try_from(b).unwrap();
                    assert!(SyncStatus::try_from(c).is_ok());
                    let vin_gid_sync = SyncStatus::try_from(c).unwrap();

                    assert_eq!(
                        vam,
                        Ok(VehicleAnnouncementMessage {
                            vin: [
                                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b,
                                0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11
                            ],
                            logical_address: [0x12, 0x13],
                            eid: [0x14, 0x15, 0x16, 0x17, 0x18, 0x19],
                            gid: [0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f],
                            further_action,
                            vin_gid_sync: Some(vin_gid_sync)
                        })
                    )
                }
                (0x11..=u8::MAX, _) => {
                    assert_eq!(vam.unwrap_err(), "Invalid ActionCode.")
                }
                (_, 0x11..=u8::MAX) => {
                    assert_eq!(vam.unwrap_err(), "Invalid SyncStatus.")
                }
            };
        }
    }

    #[test]
    fn test_try_from_bytes_short() {
        for a in u8::MIN..=u8::MAX {
            let bytes: &[u8] = &[
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
                0x1d, 0x1e, 0x1f, a,
            ];
            let vam = VehicleAnnouncementMessage::try_from(bytes);

            match a {
                0x00..=0x10 => assert_eq!(
                    vam.unwrap(),
                    VehicleAnnouncementMessage {
                        vin: [
                            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c,
                            0x0d, 0x0e, 0x0f, 0x10, 0x11
                        ],
                        logical_address: [0x12, 0x13],
                        eid: [0x14, 0x15, 0x16, 0x17, 0x18, 0x19],
                        gid: [0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f],
                        further_action: ActionCode::try_from(a).unwrap(),
                        vin_gid_sync: None
                    }
                ),
                0x11..=u8::MAX => {
                    assert_eq!(vam.unwrap_err(), "Invalid ActionCode.")
                }
            };
        }
    }

    #[test]
    fn test_vehicle_announcement_to_bytes_success_sync() {
        const N: usize = 33;

        let veh_ann_msg = VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x00, 0x00],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: Some(SyncStatus::VinGidSynchronized),
        };

        let result = <[u8; N]>::try_from(veh_ann_msg);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), [0u8; 33]);
    }

    #[test]
    fn test_vehicle_announcement_to_bytes_success_no_sync() {
        const N: usize = 32;

        let veh_ann_msg = VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x00, 0x00],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: None,
        };

        let result = <[u8; N]>::try_from(veh_ann_msg);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), [0u8; 32]);
    }

    #[test]
    fn test_vehicle_announcement_to_bytes_fail_no_sync() {
        const N: usize = 31;

        let veh_ann_msg = VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x01, 0x02],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: None,
        };

        let result = <[u8; N]>::try_from(veh_ann_msg);

        assert_eq!(result.unwrap_err(), "Invalid buffer size");
    }

    #[test]
    fn test_vehicle_announcement_to_bytes_fail_sync() {
        const N: usize = 32;

        let veh_ann_msg = VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x01, 0x02],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: Some(SyncStatus::VinGidSynchronized),
        };

        let result = <[u8; N]>::try_from(veh_ann_msg);

        assert_eq!(result.unwrap_err(), "Invalid buffer size");
    }

    #[test]
    fn test_vehicle_announcement_to_bytes_sucess_sync_large_buffer() {
        const N: usize = 35;

        let veh_ann_msg = VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x00, 0x00],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: Some(SyncStatus::VinGidSynchronized),
        };

        let result = <[u8; N]>::try_from(veh_ann_msg);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), [0u8; 35]);
    }

    #[test]
    fn test_vehicle_announcement_to_bytes_sucess_no_sync_large_buffer() {
        const N: usize = 35;

        let veh_ann_msg = VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x00, 0x00],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: None,
        };

        let result = <[u8; N]>::try_from(veh_ann_msg);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), [0u8; 35]);
    }

    #[test]
    fn test_vehicle_announcement_to_bytes_fail_size() {
        // Case 1: N is too small for even the short format
        const N1: usize = 1;

        let veh_ann_msg_short = VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x01, 0x02],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: None,
        };

        let result_short = <[u8; N1]>::try_from(veh_ann_msg_short);

        assert_eq!(result_short.unwrap_err(), "Invalid buffer size");

        // Case 2: N is enough for the short format but not for the long format
        const N2: usize = DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT;

        let veh_ann_msg_long = VehicleAnnouncementMessage {
            vin: [0u8; DOIP_COMMON_VIN_LEN],
            logical_address: [0x01, 0x02],
            eid: [0u8; DOIP_COMMON_EID_LEN],
            gid: [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: Some(SyncStatus::VinGidSynchronized),
        };

        let result_long = <[u8; N2]>::try_from(veh_ann_msg_long);

        assert_eq!(result_long.unwrap_err(), "Invalid buffer size");
    }
}
