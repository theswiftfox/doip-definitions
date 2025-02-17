/// The synchronisation status of the VIN and the GID for the entity
///
/// This gives the status that all `DoIP` entities have synchronised their information
/// about the VIN or GID of the vehicle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SyncStatus {
    /// VIN/GID Synchronized
    VinGidSynchronized = 0x00,

    /// Reserved By ISO-13400 for byte value `01`
    ReservedByIso13400_01 = 0x01,

    /// Reserved By ISO-13400 for byte value `02`
    ReservedByIso13400_02 = 0x02,

    /// Reserved By ISO-13400 for byte value `03`
    ReservedByIso13400_03 = 0x03,

    /// Reserved By ISO-13400 for byte value `04`
    ReservedByIso13400_04 = 0x04,

    /// Reserved By ISO-13400 for byte value `05`
    ReservedByIso13400_05 = 0x05,

    /// Reserved By ISO-13400 for byte value `06`
    ReservedByIso13400_06 = 0x06,

    /// Reserved By ISO-13400 for byte value `07`
    ReservedByIso13400_07 = 0x07,

    /// Reserved By ISO-13400 for byte value `08`
    ReservedByIso13400_08 = 0x08,

    /// Reserved By ISO-13400 for byte value `09`
    ReservedByIso13400_09 = 0x09,

    /// Reserved By ISO-13400 for byte value `0A`
    ReservedByIso13400_0A = 0x0A,

    /// Reserved By ISO-13400 for byte value `0B`
    ReservedByIso13400_0B = 0x0B,

    /// Reserved By ISO-13400 for byte value `0C`
    ReservedByIso13400_0C = 0x0C,

    /// Reserved By ISO-13400 for byte value `0D`
    ReservedByIso13400_0D = 0x0D,

    /// Reserved By ISO-13400 for byte value `0E`
    ReservedByIso13400_0E = 0x0E,

    /// Reserved By ISO-13400 for byte value `0F`
    ReservedByIso13400_0F = 0x0F,

    /// VVIN/GID Not Synchronised
    VinGidNotSynchronised = 0x10,
}

impl From<SyncStatus> for u8 {
    fn from(sync_status: SyncStatus) -> Self {
        sync_status as u8
    }
}

impl TryFrom<u8> for SyncStatus {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(SyncStatus::VinGidSynchronized),
            0x01 => Ok(SyncStatus::ReservedByIso13400_01),
            0x02 => Ok(SyncStatus::ReservedByIso13400_02),
            0x03 => Ok(SyncStatus::ReservedByIso13400_03),
            0x04 => Ok(SyncStatus::ReservedByIso13400_04),
            0x05 => Ok(SyncStatus::ReservedByIso13400_05),
            0x06 => Ok(SyncStatus::ReservedByIso13400_06),
            0x07 => Ok(SyncStatus::ReservedByIso13400_07),
            0x08 => Ok(SyncStatus::ReservedByIso13400_08),
            0x09 => Ok(SyncStatus::ReservedByIso13400_09),
            0x0A => Ok(SyncStatus::ReservedByIso13400_0A),
            0x0B => Ok(SyncStatus::ReservedByIso13400_0B),
            0x0C => Ok(SyncStatus::ReservedByIso13400_0C),
            0x0D => Ok(SyncStatus::ReservedByIso13400_0D),
            0x0E => Ok(SyncStatus::ReservedByIso13400_0E),
            0x0F => Ok(SyncStatus::ReservedByIso13400_0F),
            0x10 => Ok(SyncStatus::VinGidNotSynchronised),
            _ => Err("Invalid Sync Status."),
        }
    }
}
