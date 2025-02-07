/// Used in Vehicle Announcement Messages to give next steps.
///
/// Used to inform the client of further actions which need to be taken on a
/// DoIP server.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActionCode {
    /// No Further Action Required
    NoFurtherActionRequired = 0x00,

    /// Reserved By ISO-13400 for bytes value `01`
    ReservedByIso13400_01 = 0x01,

    /// Reserved By ISO-13400 for bytes value `02`
    ReservedByIso13400_02 = 0x02,

    /// Reserved By ISO-13400 for bytes value `03`
    ReservedByIso13400_03 = 0x03,

    /// Reserved By ISO-13400 for bytes value `04`
    ReservedByIso13400_04 = 0x04,

    /// Reserved By ISO-13400 for bytes value `05`
    ReservedByIso13400_05 = 0x05,

    /// Reserved By ISO-13400 for bytes value `06`
    ReservedByIso13400_06 = 0x06,

    /// Reserved By ISO-13400 for bytes value `07`
    ReservedByIso13400_07 = 0x07,

    /// Reserved By ISO-13400 for bytes value `08`
    ReservedByIso13400_08 = 0x08,

    /// Reserved By ISO-13400 for bytes value `09`
    ReservedByIso13400_09 = 0x09,

    /// Reserved By ISO-13400 for bytes value `0A`
    ReservedByIso13400_0A = 0x0A,

    /// Reserved By ISO-13400 for bytes value `0B`
    ReservedByIso13400_0B = 0x0B,

    /// Reserved By ISO-13400 for bytes value `0C`
    ReservedByIso13400_0C = 0x0C,

    /// Reserved By ISO-13400 for bytes value `0D`
    ReservedByIso13400_0D = 0x0D,

    /// Reserved By ISO-13400 for bytes value `0E`
    ReservedByIso13400_0E = 0x0E,

    /// Reserved By ISO-13400 for bytes value `0F`
    ReservedByIso13400_0F = 0x0F,

    /// Routing Activation Required
    RoutingActivationRequired = 0x10,
}
