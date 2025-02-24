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
            _ => Err("Invalid SyncStatus."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SyncStatus;

    #[test]
    fn test_try_from_bytes() {
        for n in u8::MIN..u8::MAX {
            let sync_status = SyncStatus::try_from(n);

            match n {
                0x00 => assert_eq!(sync_status.unwrap(), SyncStatus::VinGidSynchronized),
                0x01 => assert_eq!(sync_status.unwrap(), SyncStatus::ReservedByIso13400_01),
                0x02 => assert_eq!(sync_status.unwrap(), SyncStatus::ReservedByIso13400_02),
                0x03 => assert_eq!(sync_status.unwrap(), SyncStatus::ReservedByIso13400_03),
                0x04 => assert_eq!(sync_status.unwrap(), SyncStatus::ReservedByIso13400_04),
                0x05 => assert_eq!(sync_status.unwrap(), SyncStatus::ReservedByIso13400_05),
                0x06 => assert_eq!(sync_status.unwrap(), SyncStatus::ReservedByIso13400_06),
                0x07 => assert_eq!(sync_status.unwrap(), SyncStatus::ReservedByIso13400_07),
                0x08 => assert_eq!(sync_status.unwrap(), SyncStatus::ReservedByIso13400_08),
                0x09 => assert_eq!(sync_status.unwrap(), SyncStatus::ReservedByIso13400_09),
                0x0A => assert_eq!(sync_status.unwrap(), SyncStatus::ReservedByIso13400_0A),
                0x0B => assert_eq!(sync_status.unwrap(), SyncStatus::ReservedByIso13400_0B),
                0x0C => assert_eq!(sync_status.unwrap(), SyncStatus::ReservedByIso13400_0C),
                0x0D => assert_eq!(sync_status.unwrap(), SyncStatus::ReservedByIso13400_0D),
                0x0E => assert_eq!(sync_status.unwrap(), SyncStatus::ReservedByIso13400_0E),
                0x0F => assert_eq!(sync_status.unwrap(), SyncStatus::ReservedByIso13400_0F),
                0x10 => assert_eq!(sync_status.unwrap(), SyncStatus::VinGidNotSynchronised),
                _ => assert_eq!(sync_status.unwrap_err(), "Invalid SyncStatus."),
            };
        }
    }

    #[test]
    fn test_from_action_code() {
        let u = u8::from(SyncStatus::VinGidSynchronized);
        assert_eq!(u, 0x00);
        let u = u8::from(SyncStatus::ReservedByIso13400_01);
        assert_eq!(u, 0x01);
        let u = u8::from(SyncStatus::ReservedByIso13400_02);
        assert_eq!(u, 0x02);
        let u = u8::from(SyncStatus::ReservedByIso13400_03);
        assert_eq!(u, 0x03);
        let u = u8::from(SyncStatus::ReservedByIso13400_04);
        assert_eq!(u, 0x04);
        let u = u8::from(SyncStatus::ReservedByIso13400_05);
        assert_eq!(u, 0x05);
        let u = u8::from(SyncStatus::ReservedByIso13400_06);
        assert_eq!(u, 0x06);
        let u = u8::from(SyncStatus::ReservedByIso13400_07);
        assert_eq!(u, 0x07);
        let u = u8::from(SyncStatus::ReservedByIso13400_08);
        assert_eq!(u, 0x08);
        let u = u8::from(SyncStatus::ReservedByIso13400_09);
        assert_eq!(u, 0x09);
        let u = u8::from(SyncStatus::ReservedByIso13400_0A);
        assert_eq!(u, 0x0A);
        let u = u8::from(SyncStatus::ReservedByIso13400_0B);
        assert_eq!(u, 0x0B);
        let u = u8::from(SyncStatus::ReservedByIso13400_0C);
        assert_eq!(u, 0x0C);
        let u = u8::from(SyncStatus::ReservedByIso13400_0D);
        assert_eq!(u, 0x0D);
        let u = u8::from(SyncStatus::ReservedByIso13400_0E);
        assert_eq!(u, 0x0E);
        let u = u8::from(SyncStatus::ReservedByIso13400_0F);
        assert_eq!(u, 0x0F);
        let u = u8::from(SyncStatus::VinGidNotSynchronised);
        assert_eq!(u, 0x10);
    }
}
