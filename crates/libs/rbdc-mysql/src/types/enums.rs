use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

use rbdc::Error;

use crate::io::MySqlBufMutExt;
use crate::types::Decode;
use crate::types::Encode;
use crate::value::MySqlValue;

#[derive(serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq)]
#[serde(rename = "Enum")]
pub struct Enum(pub String);

impl Display for Enum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for Enum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Enum({})", self.0)
    }
}

impl Encode for Enum {
    fn encode(self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        let bytes = self.0.into_bytes();
        let len = bytes.len();
        buf.put_bytes_lenenc(bytes);
        Ok(len)
    }
}

impl Decode for Enum {
    fn decode(value: MySqlValue) -> Result<Self, Error> {
        Ok(Self(value.as_str().unwrap_or_default().to_string()))
    }
}
