use crate::payload::PowerMode;

/// Expected reponse from `PowerInformationRequest`.
///
/// Containing details of the target of the `PowerInformationRequest`, the
/// `EntityStatusResponse` provides the program with details pertaining to the
/// active power mode status of the entity.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PowerInformationResponse {
    /// Possible power modes available
    pub power_mode: PowerMode,
}

impl From<PowerInformationResponse> for [u8; 1] {
    fn from(power_information_response: PowerInformationResponse) -> Self {
        [power_information_response.power_mode as u8]
    }
}

impl TryFrom<[u8; 1]> for PowerInformationResponse {
    type Error = &'static str;

    fn try_from(value: [u8; 1]) -> Result<Self, Self::Error> {
        let power_mode = PowerMode::try_from(value[0])?;
        Ok(PowerInformationResponse { power_mode })
    }
}

#[cfg(test)]
mod tests {
    use crate::payload::PowerMode;

    use super::PowerInformationResponse;

    #[test]
    fn test_try_from_bytes() {
        for a in u8::MIN..u8::MAX {
            let power_res = PowerInformationResponse::try_from([a]);

            match a {
                0x00 => assert_eq!(
                    power_res.unwrap(),
                    PowerInformationResponse {
                        power_mode: PowerMode::NotReady
                    }
                ),
                0x01 => assert_eq!(
                    power_res.unwrap(),
                    PowerInformationResponse {
                        power_mode: PowerMode::Ready
                    }
                ),
                0x02 => assert_eq!(
                    power_res.unwrap(),
                    PowerInformationResponse {
                        power_mode: PowerMode::NotSupported
                    }
                ),
                _ => assert_eq!(power_res.unwrap_err(), "Invalid PowerMode."),
            };
        }
    }

    #[test]
    fn test_from_power_info_res() {
        let power_info_res = PowerInformationResponse {
            power_mode: PowerMode::Ready,
        };
        let power_info_res_bytes = <[u8; 1]>::from(power_info_res);

        assert_eq!(power_info_res_bytes, [0x01]);
    }
}
