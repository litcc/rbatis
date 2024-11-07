use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

use byteorder::ByteOrder;
use byteorder::LittleEndian;
use rbdc::Error;

use crate::types::Decode;
use crate::types::Encode;
use crate::value::MySqlValue;
use crate::value::MySqlValueFormat;

#[derive(serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq)]
#[serde(rename = "Year")]
pub struct Year(pub u16);

impl Display for Year {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for Year {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Year({})", self.0)
    }
}

impl Encode for Year {
    fn encode(self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        buf.push(2);
        buf.extend_from_slice(&self.0.to_le_bytes());
        Ok(2)
    }
}

impl Decode for Year {
    fn decode(value: MySqlValue) -> Result<Self, Error> {
        Ok(Self({
            match value.format() {
                MySqlValueFormat::Text => {
                    value.as_str()?.parse().unwrap_or_default()
                }
                MySqlValueFormat::Binary => {
                    let buf = value.as_bytes()?;
                    LittleEndian::read_u16(&buf[1..])
                }
            }
        }))
    }
}
