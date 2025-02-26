use crate::{
    definitions::{
        DOIP_COMMON_EID_LEN, DOIP_COMMON_VIN_LEN, DOIP_DIAG_COMMON_SOURCE_LEN,
        DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN,
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
