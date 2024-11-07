use rbdc::error::Error;

use crate::decode::Decode;
use crate::encode::Encode;
use crate::encode::IsNull;
use crate::type_info::DataType;
use crate::types::Type;
use crate::SqliteArgumentValue;
use crate::SqliteTypeInfo;
use crate::SqliteValue;

impl Type for Vec<u8> {
    fn type_info(&self) -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Blob)
    }
}

impl Encode for Vec<u8> {
    fn encode(self, args: &mut Vec<SqliteArgumentValue>) -> Result<IsNull, Error> {
        args.push(SqliteArgumentValue::Blob(self));

        Ok(IsNull::No)
    }
}

impl Decode for Vec<u8> {
    fn decode(value: SqliteValue) -> Result<Self, Error> {
        Ok(value.blob().to_owned())
    }
}
