use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::{self};
use std::os::raw::c_int;
use std::str::FromStr;

use libsqlite3_sys::SQLITE_BLOB;
use libsqlite3_sys::SQLITE_FLOAT;
use libsqlite3_sys::SQLITE_INTEGER;
use libsqlite3_sys::SQLITE_NULL;
use libsqlite3_sys::SQLITE_TEXT;
use rbdc::Error;
use rbs::Value;

pub trait Type {
    fn type_info(&self) -> SqliteTypeInfo;
}

// for optionals, the underlying SQL type is identical
impl<T: Type> Type for Option<T> {
    fn type_info(&self) -> SqliteTypeInfo {
        match self {
            None => SqliteTypeInfo(DataType::Null),
            Some(v) => v.type_info(),
        }
    }
}

#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    serde::Serialize,
    serde::Deserialize
)]
pub(crate) enum DataType {
    Null,
    Int,
    Float,
    Text,
    Blob,

    // TODO: Support NUMERIC
    #[allow(dead_code)]
    Numeric,

    // non-standard extensions
    Bool,
    Int64,
    Date,
    Time,
    Datetime,
}

/// Type information for a SQLite type.
#[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SqliteTypeInfo(pub(crate) DataType);

impl SqliteTypeInfo {
    pub fn null() -> Self {
        SqliteTypeInfo(DataType::Null)
    }
}

impl Display for SqliteTypeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(self.name())
    }
}

impl SqliteTypeInfo {
    pub fn is_null(&self) -> bool {
        matches!(self.0, DataType::Null)
    }

    pub fn name(&self) -> &str {
        match self.0 {
            DataType::Null => "NULL",
            DataType::Text => "TEXT",
            DataType::Float => "REAL",
            DataType::Blob => "BLOB",
            DataType::Int | DataType::Int64 => "INTEGER",
            DataType::Numeric => "NUMERIC",

            // non-standard extensions
            DataType::Bool => "BOOLEAN",
            DataType::Date => "DATE",
            DataType::Time => "TIME",
            DataType::Datetime => "DATETIME",
        }
    }
}

impl DataType {
    pub(crate) fn from_code(code: c_int) -> Self {
        match code {
            SQLITE_INTEGER => DataType::Int,
            SQLITE_FLOAT => DataType::Float,
            SQLITE_BLOB => DataType::Blob,
            SQLITE_NULL => DataType::Null,
            SQLITE_TEXT => DataType::Text,

            // https://sqlite.org/c3ref/c_blob.html
            _ => panic!("unknown data type code {}", code),
        }
    }
}

// note: this implementation is particularly important as this is how the macros
// determine       what Rust type maps to what *declared* SQL type
// <https://www.sqlite.org/datatype3.html#affname>
impl FromStr for DataType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_ascii_lowercase();
        Ok(match &*s {
            "int4" => DataType::Int,
            "int8" => DataType::Int64,
            "boolean" | "bool" => DataType::Bool,

            "date" => DataType::Date,
            "time" => DataType::Time,
            "datetime" | "timestamp" => DataType::Datetime,

            _ if s.contains("int") => DataType::Int64,

            _ if s.contains("char") || s.contains("clob") || s.contains("text") => {
                DataType::Text
            }

            _ if s.contains("blob") => DataType::Blob,

            _ if s.contains("real") || s.contains("floa") || s.contains("doub") => {
                DataType::Float
            }

            _ => {
                return Err(format!("unknown type: `{}`", s).into());
            }
        })
    }
}

impl Type for Value {
    fn type_info(&self) -> SqliteTypeInfo {
        match self {
            Value::Null => SqliteTypeInfo::null(),
            Value::SomeNull => SqliteTypeInfo::null(),
            Value::Bool(_) => SqliteTypeInfo(DataType::Bool),
            Value::I32(_) => SqliteTypeInfo(DataType::Int),
            Value::I64(_) => SqliteTypeInfo(DataType::Int64),
            Value::U32(_) => SqliteTypeInfo(DataType::Int),
            Value::U64(_) => SqliteTypeInfo(DataType::Int64),
            Value::F32(_) => SqliteTypeInfo(DataType::Float),
            Value::F64(_) => SqliteTypeInfo(DataType::Float),
            Value::String(_) => SqliteTypeInfo(DataType::Text),
            Value::Binary(_) => SqliteTypeInfo(DataType::Blob),
            Value::Array(_) => SqliteTypeInfo(DataType::Null),
            Value::Map(_) => SqliteTypeInfo(DataType::Null),
            Value::Ext(t, _) => match *t {
                "Date" => SqliteTypeInfo(DataType::Text),
                "DateTime" => SqliteTypeInfo(DataType::Text),
                "Time" => SqliteTypeInfo(DataType::Text),
                "Timestamp" => SqliteTypeInfo(DataType::Int64),
                "Decimal" => SqliteTypeInfo(DataType::Numeric),
                "Json" => SqliteTypeInfo(DataType::Blob),
                "Uuid" => SqliteTypeInfo(DataType::Text),
                _ => SqliteTypeInfo(DataType::Null),
            },
        }
    }
}

#[test]
fn test_data_type_from_str() -> Result<(), Error> {
    assert_eq!(DataType::Int, "INT4".parse()?);

    assert_eq!(DataType::Int64, "INT".parse()?);
    assert_eq!(DataType::Int64, "INTEGER".parse()?);
    assert_eq!(DataType::Int64, "INTBIG".parse()?);
    assert_eq!(DataType::Int64, "MEDIUMINT".parse()?);

    assert_eq!(DataType::Int64, "BIGINT".parse()?);
    assert_eq!(DataType::Int64, "UNSIGNED BIG INT".parse()?);
    assert_eq!(DataType::Int64, "INT8".parse()?);

    assert_eq!(DataType::Text, "CHARACTER(20)".parse()?);
    assert_eq!(DataType::Text, "NCHAR(55)".parse()?);
    assert_eq!(DataType::Text, "TEXT".parse()?);
    assert_eq!(DataType::Text, "CLOB".parse()?);

    assert_eq!(DataType::Blob, "BLOB".parse()?);

    assert_eq!(DataType::Float, "REAL".parse()?);
    assert_eq!(DataType::Float, "FLOAT".parse()?);
    assert_eq!(DataType::Float, "DOUBLE PRECISION".parse()?);

    assert_eq!(DataType::Bool, "BOOLEAN".parse()?);
    assert_eq!(DataType::Bool, "BOOL".parse()?);

    assert_eq!(DataType::Datetime, "DATETIME".parse()?);
    assert_eq!(DataType::Time, "TIME".parse()?);
    assert_eq!(DataType::Date, "DATE".parse()?);

    Ok(())
}
