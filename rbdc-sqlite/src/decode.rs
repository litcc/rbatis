use rbdc::Error;

use crate::SqliteValue;

pub trait Decode {
    fn decode(value: SqliteValue) -> Result<Self, Error>
    where
        Self: Sized;
}
