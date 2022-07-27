use std::str::FromStr;
use std::time::Duration;
use rbdc::datetime::DateTime;
use rbdc::Error;
use rbdc::timestamp::Timestamp;
use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::{Encode, IsNull};
use crate::value::{PgValue, PgValueFormat};

impl Encode for DateTime {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull,Error> {
        self.0.encode(buf)?;
        Ok(IsNull::No)
    }
}

impl Decode for DateTime {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(Self(fastdate::DateTime::decode(value)?))
    }
}

impl Decode for fastdate::DateTime {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                // TIMESTAMP is encoded as the microseconds since the epoch
                let epoch = fastdate::DateTime {
                    micro: 0,
                    sec: 0,
                    min: 0,
                    hour: 0,
                    day: 1,
                    mon: 1,
                    year: 2000,
                };
                let us: i64 = Decode::decode(value)?;
                epoch + Duration::from_micros(us as u64)
            }
            PgValueFormat::Text => {
                //2022-07-22 05:22:22.123456+00
                let s = value.as_str()?;
                let bytes = s.as_bytes();
                if bytes[bytes.len() - 3] == '+' as u8 {
                    //have zone
                    let mut dt = fastdate::DateTime::from_str(&s[0..s.len() - 3])
                        .map_err(|e| Error::from(e.to_string()))?;
                    let hour: i32 = s[s.len() - 2..s.len()].parse().unwrap_or_default();
                    dt = dt + Duration::from_secs((hour * 3600) as u64);
                    dt
                } else {
                    fastdate::DateTime::from_str(s).map_err(|e| Error::from(e.to_string()))?
                }
            }
        })
    }
}

impl Encode for fastdate::DateTime {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull,Error> {
        Timestamp(self.unix_timestamp_millis() as u64).encode(buf)
    }
}