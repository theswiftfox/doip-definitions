use crate::error::{Error, Result};
use crate::payload;
use crate::{header::DoipHeader, message::DoipMessage, payload::DoipPayload};

#[derive(Default, Debug)]
pub struct DoipMessageBuilder {
    header: Option<DoipHeader>,
    payload: Option<DoipPayload>,
}

impl DoipMessageBuilder {
    pub fn new() -> Self {
        DoipMessageBuilder::default()
    }

    pub fn header(&mut self, header: impl Into<DoipHeader>) -> &mut Self {
        self.header = Some(header.into());
        self
    }

    pub fn payload(&mut self, payload: impl Into<DoipPayload>) -> &mut Self {
        self.payload = Some(payload.into());
        self
    }

    fn build(&self) -> Result<DoipMessage> {
        let Some(header) = self.header.as_ref() else {
            return Err(Error::HeaderNotBuilt);
        };

        let Some(payload) = self.payload.as_ref() else {
            return Err(Error::PayloadNotBuilt);
        };

        Ok(DoipMessage {
            header: header.clone(),
            payload: payload.clone(),
        })
    }
}
