use rbdc::error::Error;

use crate::{
    decode::Decode,
    encode::{Encode, IsNull},
    type_info::DataType,
    types::Type,
    SqliteArgumentValue, SqliteTypeInfo, SqliteValue,
};

impl Type for f32 {
    fn type_info(&self) -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Float)
    }
}

impl Encode for f32 {
    fn encode(self, args: &mut Vec<SqliteArgumentValue>) -> Result<IsNull, Error> {
        args.push(SqliteArgumentValue::Double(self.into()));

        Ok(IsNull::No)
    }
}

impl Decode for f32 {
    fn decode(value: SqliteValue) -> Result<f32, Error> {
        Ok(value.double() as f32)
    }
}

impl Type for f64 {
    fn type_info(&self) -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Float)
    }
}

impl Encode for f64 {
    fn encode(self, args: &mut Vec<SqliteArgumentValue>) -> Result<IsNull, Error> {
        args.push(SqliteArgumentValue::Double(self));

        Ok(IsNull::No)
    }
}

impl Decode for f64 {
    fn decode(value: SqliteValue) -> Result<f64, Error> {
        Ok(value.double())
    }
}
