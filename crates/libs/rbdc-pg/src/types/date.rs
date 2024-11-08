use std::str::FromStr;
use std::time::Duration;

use rbdc::date::Date;
use rbdc::Error;

use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::Encode;
use crate::types::encode::IsNull;
use crate::value::PgValue;
use crate::value::PgValueFormat;

impl Decode for fastdate::Date {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                // DATE is encoded as the days since epoch
                let days: i32 = Decode::decode(value)?;
                let dt = fastdate::DateTime::from((
                    fastdate::Date { day: 1, mon: 1, year: 2000 },
                    fastdate::Time { nano: 0, sec: 0, minute: 0, hour: 0 },
                ));
                let dt = {
                    if days < 0 {
                        dt - Duration::from_secs((-days * 24 * 3600) as u64)
                    } else {
                        dt + Duration::from_secs((days * 24 * 3600) as u64)
                    }
                };
                fastdate::Date::from(dt)
            }

            PgValueFormat::Text => {
                let dt = fastdate::DateTime::from_str(&format!(
                    "{}T00:00:00Z",
                    value.as_str()?
                ))
                .map_err(|e| Error::from(e.to_string()))?;
                fastdate::Date::from(dt)
            }
        })
    }
}

impl Encode for fastdate::Date {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        // DATE is encoded as the days since epoch

        let days = (fastdate::DateTime::from((
            fastdate::Date { day: self.day, mon: self.mon, year: self.year },
            fastdate::Time { nano: 0, sec: 0, minute: 0, hour: 0 },
        ))
        .unix_timestamp_millis() -
            fastdate::DateTime::from((
                fastdate::Date { day: 1, mon: 1, year: 2000 },
                fastdate::Time { nano: 0, sec: 0, minute: 0, hour: 0 },
            ))
            .unix_timestamp_millis()) /
            (86400 * 1000) as i64;
        (days as i32).encode(buf)
    }
}

impl Decode for Date {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(Self(fastdate::Date::decode(value)?))
    }
}

impl Encode for Date {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        self.0.encode(buf)
    }
}
