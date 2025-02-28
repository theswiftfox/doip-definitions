/// Checks the TCP Socket is still alive
///
/// Sent with no payload, the `AliveCheckRequest` is utilised to maintain a connection
/// to a TCP socket or to check the status of one.
#[cfg_attr(feature = "std", pyo3::pyclass)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AliveCheckRequest {}
