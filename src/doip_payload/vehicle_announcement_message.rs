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

impl TryFrom<[u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG]> for VehicleAnnouncementMessage {
    type Error = &'static str;

    fn try_from(value: [u8; DOIP_VEHICLE_ANNOUNCEMENT_LEN_LONG]) -> Result<Self, Self::Error> {
        let (vin_slice, rest) = value.split_at(DOIP_COMMON_VIN_LEN);
        let (logical_address_slice, rest) = rest.split_at(DOIP_DIAG_COMMON_SOURCE_LEN);
        let (eid_slice, rest) = rest.split_at(DOIP_COMMON_EID_LEN);
        let (gid_slice, rest) = rest.split_at(DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN);
        let (further_action_slice, rest) = rest.split_at(1);
        let (vin_gid_sync_slice, _) = rest.split_at(1);

        let vin: [u8; DOIP_COMMON_VIN_LEN] =
            vin_slice.try_into().map_err(|_| "Invalid vin length")?;

        let logical_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN] =
            logical_address_slice
                .try_into()
                .map_err(|_| "Invalid logical address length")?;

        let eid: [u8; DOIP_COMMON_EID_LEN] =
            eid_slice.try_into().map_err(|_| "Invalid eid length")?;

        let gid: [u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN] =
            gid_slice.try_into().map_err(|_| "Invalid gid length")?;

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

        let vin: [u8; DOIP_COMMON_VIN_LEN] =
            vin_slice.try_into().map_err(|_| "Invalid vin length")?;

        let logical_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN] =
            logical_address_slice
                .try_into()
                .map_err(|_| "Invalid logical address length")?;

        let eid: [u8; DOIP_COMMON_EID_LEN] =
            eid_slice.try_into().map_err(|_| "Invalid eid length")?;

        let gid: [u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN] =
            gid_slice.try_into().map_err(|_| "Invalid gid length")?;

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
