use rbdc::{uuid::Uuid, Error};

use crate::{
    io::MySqlBufMutExt,
    types::{Decode, Encode},
    value::MySqlValue,
};

impl Encode for Uuid {
    fn encode(self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        let bytes = self.0.into_bytes();
        let len = bytes.len();
        buf.put_bytes_lenenc(bytes);
        Ok(len)
    }
}

impl Decode for Uuid {
    fn decode(value: MySqlValue) -> Result<Self, Error> {
        Ok(Self(value.as_str().unwrap_or_default().to_string()))
    }
}
