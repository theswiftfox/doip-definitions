/// Used in `PowerInformationResponse`, `PowerMode` provides the power mode that
/// the `DoIP` entity can be.

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PowerMode {
    /// Not Ready
    NotReady = 0x00,

    /// Ready
    Ready = 0x01,

    /// Not Supported
    NotSupported = 0x02,
}
