use rbdc::error::Error;

use crate::decode::Decode;
use crate::encode::Encode;
use crate::encode::IsNull;
use crate::type_info::DataType;
use crate::types::Type;
use crate::SqliteArgumentValue;
use crate::SqliteTypeInfo;
use crate::SqliteValue;

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
