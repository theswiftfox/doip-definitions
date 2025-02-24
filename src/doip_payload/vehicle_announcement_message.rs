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

impl From<VehicleAnnouncementMessage> for [u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG] {
    fn from(vehicle_announcement_message: VehicleAnnouncementMessage) -> Self {
        let vin = vehicle_announcement_message.vin;
        let logical_address = vehicle_announcement_message.logical_address;
        let eid = vehicle_announcement_message.eid;
        let gid = vehicle_announcement_message.gid;
        let further_action = [u8::from(vehicle_announcement_message.further_action)];

        let vin_gid_sync = match vehicle_announcement_message.vin_gid_sync {
            Some(sync_status) => [u8::from(sync_status)],
            None => [0],
        };

        let mut buffer = [0; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG];
        let mut offset = 0;

        buffer[offset..offset + DOIP_COMMON_VIN_LEN].copy_from_slice(&vin);
        offset += DOIP_COMMON_VIN_LEN;

        buffer[offset..offset + DOIP_DIAG_COMMON_SOURCE_LEN].copy_from_slice(&logical_address);
        offset += DOIP_DIAG_COMMON_SOURCE_LEN;

        buffer[offset..offset + DOIP_COMMON_EID_LEN].copy_from_slice(&eid);
        offset += DOIP_COMMON_EID_LEN;

        buffer[offset..offset + DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN].copy_from_slice(&gid);
        offset += DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN;

        buffer[offset] = further_action[0];
        offset += 1;

        buffer[offset] = vin_gid_sync[0];

        buffer
    }
}

impl From<VehicleAnnouncementMessage> for [u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT] {
    fn from(vehicle_announcement_message: VehicleAnnouncementMessage) -> Self {
        let vin = vehicle_announcement_message.vin;
        let logical_address = vehicle_announcement_message.logical_address;
        let eid = vehicle_announcement_message.eid;
        let gid = vehicle_announcement_message.gid;
        let further_action = [u8::from(vehicle_announcement_message.further_action)];

        let mut buffer = [0; DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT];
        let mut offset = 0;

        buffer[offset..offset + DOIP_COMMON_VIN_LEN].copy_from_slice(&vin);
        offset += DOIP_COMMON_VIN_LEN;

        buffer[offset..offset + DOIP_DIAG_COMMON_SOURCE_LEN].copy_from_slice(&logical_address);
        offset += DOIP_DIAG_COMMON_SOURCE_LEN;

        buffer[offset..offset + DOIP_COMMON_EID_LEN].copy_from_slice(&eid);
        offset += DOIP_COMMON_EID_LEN;

        buffer[offset..offset + DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN].copy_from_slice(&gid);
        offset += DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN;

        buffer[offset] = further_action[0];

        buffer
    }
}

impl TryFrom<[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG]> for VehicleAnnouncementMessage {
    type Error = &'static str;

    fn try_from(value: [u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG]) -> Result<Self, Self::Error> {
        let (vin_slice, rest) = value.split_at(DOIP_COMMON_VIN_LEN);
        let (logical_address_slice, rest) = rest.split_at(DOIP_DIAG_COMMON_SOURCE_LEN);
        let (eid_slice, rest) = rest.split_at(DOIP_COMMON_EID_LEN);
        let (gid_slice, rest) = rest.split_at(DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN);
        let (further_action_slice, rest) = rest.split_at(1);
        let (vin_gid_sync_slice, _) = rest.split_at(1);

        let mut vin = [0u8; DOIP_COMMON_VIN_LEN];
        vin.copy_from_slice(vin_slice);

        let mut logical_address = [0u8; DOIP_DIAG_COMMON_SOURCE_LEN];
        logical_address.copy_from_slice(logical_address_slice);

        let mut eid = [0u8; DOIP_COMMON_EID_LEN];
        eid.copy_from_slice(eid_slice);

        let mut gid = [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN];
        gid.copy_from_slice(gid_slice);

        let further_action = ActionCode::try_from(further_action_slice[0])?;

        let vin_gid_sync = Some(SyncStatus::try_from(vin_gid_sync_slice[0])?);

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

impl TryFrom<[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT]> for VehicleAnnouncementMessage {
    type Error = &'static str;

    fn try_from(value: [u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT]) -> Result<Self, Self::Error> {
        let (vin_slice, rest) = value.split_at(DOIP_COMMON_VIN_LEN);
        let (logical_address_slice, rest) = rest.split_at(DOIP_DIAG_COMMON_SOURCE_LEN);
        let (eid_slice, rest) = rest.split_at(DOIP_COMMON_EID_LEN);
        let (gid_slice, rest) = rest.split_at(DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN);
        let (further_action_slice, _) = rest.split_at(1);

        let mut vin = [0u8; DOIP_COMMON_VIN_LEN];
        vin.copy_from_slice(vin_slice);

        let mut logical_address = [0u8; DOIP_DIAG_COMMON_SOURCE_LEN];
        logical_address.copy_from_slice(logical_address_slice);

        let mut eid = [0u8; DOIP_COMMON_EID_LEN];
        eid.copy_from_slice(eid_slice);

        let mut gid = [0u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN];
        gid.copy_from_slice(gid_slice);

        let further_action = ActionCode::try_from(further_action_slice[0])?;

        let vin_gid_sync = None;

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
        definitions::{DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG, DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT},
        payload::{ActionCode, SyncStatus},
    };

    use super::VehicleAnnouncementMessage;

    #[test]
    fn test_try_from_bytes_long() {
        for a in u16::MIN..=u16::MAX {
            let (b, c) = (a.to_be_bytes()[0], a.to_be_bytes()[1]);
            let bytes: [u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG] = [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
                0x1d, 0x1e, 0x1f, b, c,
            ];
            let vam = VehicleAnnouncementMessage::try_from(bytes);

            match (b, c) {
                (0x00..=0x10, 0x00..=0x10) => assert_eq!(
                    vam.unwrap(),
                    VehicleAnnouncementMessage {
                        vin: [
                            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c,
                            0x0d, 0x0e, 0x0f, 0x10, 0x11
                        ],
                        logical_address: [0x12, 0x13],
                        eid: [0x14, 0x15, 0x16, 0x17, 0x18, 0x19],
                        gid: [0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f],
                        further_action: ActionCode::try_from(b).unwrap(),
                        vin_gid_sync: Some(SyncStatus::try_from(c).unwrap())
                    }
                ),
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
            let bytes: [u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT] = [
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
    fn test_from_vehicle_announcement_msg_long_sync() {
        let bytes: [u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG] = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
            0x1d, 0x1e, 0x1f, 0x00, 0x00,
        ];

        let vam = VehicleAnnouncementMessage {
            vin: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10, 0x11,
            ],
            logical_address: [0x12, 0x13],
            eid: [0x14, 0x15, 0x16, 0x17, 0x18, 0x19],
            gid: [0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: Some(SyncStatus::VinGidSynchronized),
        };

        assert_eq!(<[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG]>::from(vam), bytes);
    }

    #[test]
    fn test_from_vehicle_announcement_msg_long_no_sync() {
        let bytes: [u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG] = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
            0x1d, 0x1e, 0x1f, 0x00, 0x00,
        ];

        let vam = VehicleAnnouncementMessage {
            vin: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10, 0x11,
            ],
            logical_address: [0x12, 0x13],
            eid: [0x14, 0x15, 0x16, 0x17, 0x18, 0x19],
            gid: [0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: None,
        };

        assert_eq!(<[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG]>::from(vam), bytes);
    }

    #[test]
    fn test_from_vehicle_announcement_msg_short_sync() {
        let bytes: [u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT] = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
            0x1d, 0x1e, 0x1f, 0x00,
        ];

        let vam = VehicleAnnouncementMessage {
            vin: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10, 0x11,
            ],
            logical_address: [0x12, 0x13],
            eid: [0x14, 0x15, 0x16, 0x17, 0x18, 0x19],
            gid: [0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: None,
        };

        assert_eq!(
            <[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT]>::from(vam),
            bytes
        );
    }

    #[test]
    fn test_from_vehicle_announcement_msg_short_no_sync() {
        let bytes: [u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT] = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
            0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c,
            0x1d, 0x1e, 0x1f, 0x00,
        ];

        let vam = VehicleAnnouncementMessage {
            vin: [
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                0x0f, 0x10, 0x11,
            ],
            logical_address: [0x12, 0x13],
            eid: [0x14, 0x15, 0x16, 0x17, 0x18, 0x19],
            gid: [0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f],
            further_action: ActionCode::NoFurtherActionRequired,
            vin_gid_sync: None,
        };

        assert_eq!(
            <[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_SHORT]>::from(vam),
            bytes
        );
    }
}
