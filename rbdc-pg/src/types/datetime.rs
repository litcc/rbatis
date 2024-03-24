use std::{str::FromStr, time::Duration};

use rbdc::{datetime::DateTime, timestamp::Timestamp, Error};

use crate::{
    arguments::PgArgumentBuffer,
    types::{
        decode::Decode,
        encode::{Encode, IsNull},
    },
    value::{PgValue, PgValueFormat},
};

impl Encode for DateTime {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        self.0.encode(buf)?;
        Ok(IsNull::No)
    }
}

impl Decode for DateTime {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(Self(fastdate::DateTime::decode(value)?))
    }
}

/// pg only have timestamp,so time is utc time
impl Decode for fastdate::DateTime {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                // TIMESTAMP is encoded as the microseconds since the epoch
                let mut epoch = fastdate::DateTime::from(fastdate::Date {
                    day: 1,
                    mon: 1,
                    year: 2000,
                });
                let us: i64 = Decode::decode(value)?;
                if us < 0 {
                    epoch = epoch - Duration::from_micros(-us as u64)
                } else {
                    epoch = epoch + Duration::from_micros(us as u64)
                }
                epoch
            }
            PgValueFormat::Text => {
                //2022-07-22 05:22:22.123456+00
                fastdate::DateTime::from_str(value.as_str()?)
                    .map_err(|e| Error::from(e.to_string()))?
            }
        })
    }
}

impl Encode for fastdate::DateTime {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        Timestamp(self.unix_timestamp_millis()).encode(buf)
    }
}
