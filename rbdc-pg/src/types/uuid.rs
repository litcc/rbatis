use std::str::FromStr;

use rbdc::{uuid::Uuid, Error};

use crate::{
    arguments::PgArgumentBuffer,
    types::{
        decode::Decode,
        encode::{Encode, IsNull},
    },
    value::{PgValue, PgValueFormat},
};

impl Encode for Uuid {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        let uuid =
            uuid::Uuid::from_str(&self.0).map_err(|e| Error::from(e.to_string()))?;
        buf.extend_from_slice(uuid.as_bytes());
        Ok(IsNull::No)
    }
}

impl Decode for Uuid {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(Self(match value.format() {
            PgValueFormat::Binary => uuid::Uuid::from_slice(value.as_bytes()?)
                .map_err(|e| Error::from(format!("Decode Uuid:{}", e)))?
                .to_string(),
            PgValueFormat::Text => value
                .as_str()?
                .parse()
                .map_err(|e| Error::from(format!("Decode Uuid str:{}", e)))?,
        }))
    }
}
