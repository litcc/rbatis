use rbdc::error::Error;

use crate::decode::Decode;
use crate::encode::Encode;
use crate::encode::IsNull;
use crate::type_info::DataType;
use crate::types::Type;
use crate::SqliteArgumentValue;
use crate::SqliteTypeInfo;
use crate::SqliteValue;

impl Type for String {
    fn type_info(&self) -> SqliteTypeInfo {
        SqliteTypeInfo(DataType::Text)
    }
}

impl Encode for String {
    fn encode(self, args: &mut Vec<SqliteArgumentValue>) -> Result<IsNull, Error> {
        args.push(SqliteArgumentValue::Text(self));

        Ok(IsNull::No)
    }
}

impl Decode for String {
    fn decode(value: SqliteValue) -> Result<Self, Error> {
        value.text().map(ToOwned::to_owned)
    }
}
