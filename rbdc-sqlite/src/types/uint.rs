use rbdc::error::Error;

use crate::{
    decode::Decode,
    encode::{Encode, IsNull},
    type_info::DataType,
    types::Type,
    SqliteArgumentValue, SqliteTypeInfo, SqliteValue,
};

impl Type for u8 {
    fn type_info(&self) -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Int)
    }
}

impl Encode for u8 {
    fn encode(self, args: &mut Vec<SqliteArgumentValue>) -> Result<IsNull, Error> {
        args.push(SqliteArgumentValue::Int(self as i32));

        Ok(IsNull::No)
    }
}

impl Decode for u8 {
    fn decode(value: SqliteValue) -> Result<Self, Error> {
        Ok(value.int().try_into()?)
    }
}

impl Type for u16 {
    fn type_info(&self) -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Int)
    }
}

impl Encode for u16 {
    fn encode(self, args: &mut Vec<SqliteArgumentValue>) -> Result<IsNull, Error> {
        args.push(SqliteArgumentValue::Int(self as i32));

        Ok(IsNull::No)
    }
}

impl Decode for u16 {
    fn decode(value: SqliteValue) -> Result<Self, Error> {
        Ok(value.int().try_into()?)
    }
}

impl Type for u32 {
    fn type_info(&self) -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Int64)
    }
}

impl Encode for u32 {
    fn encode(self, args: &mut Vec<SqliteArgumentValue>) -> Result<IsNull, Error> {
        args.push(SqliteArgumentValue::Int64(self as i64));

        Ok(IsNull::No)
    }
}

impl Decode for u32 {
    fn decode(value: SqliteValue) -> Result<Self, Error> {
        Ok(value.int64().try_into()?)
    }
}
