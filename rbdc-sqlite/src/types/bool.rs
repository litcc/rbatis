use rbdc::error::Error;

use crate::{
    decode::Decode,
    encode::{Encode, IsNull},
    type_info::DataType,
    types::Type,
    SqliteArgumentValue, SqliteTypeInfo, SqliteValue,
};

impl Type for bool {
    fn type_info(&self) -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Bool)
    }
}

impl Encode for bool {
    fn encode(self, args: &mut Vec<SqliteArgumentValue>) -> Result<IsNull, Error> {
        args.push(SqliteArgumentValue::Int(i32::from(self)));

        Ok(IsNull::No)
    }
}

impl Decode for bool {
    fn decode(value: SqliteValue) -> Result<bool, Error> {
        Ok(value.int() != 0)
    }
}
