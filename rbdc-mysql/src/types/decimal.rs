use std::str::FromStr;

use rbdc::{decimal::Decimal, Error};

use crate::{
    io::MySqlBufMutExt,
    types::{Decode, Encode},
    value::MySqlValue,
};

impl Encode for Decimal {
    fn encode(self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        let bytes = self.0.to_string().into_bytes();
        let len = bytes.len();
        buf.put_bytes_lenenc(bytes);
        Ok(len)
    }
}

impl Decode for Decimal {
    fn decode(value: MySqlValue) -> Result<Self, Error> {
        Decimal::from_str(value.as_str().unwrap_or("0"))
    }
}
