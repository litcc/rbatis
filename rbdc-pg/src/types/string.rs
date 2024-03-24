use rbdc::Error;

use crate::{
    arguments::PgArgumentBuffer,
    type_info::PgTypeInfo,
    types::{
        decode::Decode,
        encode::{Encode, IsNull},
        TypeInfo,
    },
    value::PgValue,
};

impl Decode for String {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(value.as_str()?.to_owned())
    }
}

impl Encode for String {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        buf.extend(self.into_bytes());
        Ok(IsNull::No)
    }
}

impl TypeInfo for String {
    fn type_info(&self) -> PgTypeInfo {
        PgTypeInfo::VARCHAR
    }
}
