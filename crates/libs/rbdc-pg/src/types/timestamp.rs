use std::str::FromStr;

use rbdc::timestamp::Timestamp;
use rbdc::Error;

use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::Encode;
use crate::types::encode::IsNull;
use crate::value::PgValue;
use crate::value::PgValueFormat;

impl Encode for Timestamp {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        let epoch =
            fastdate::DateTime::from(fastdate::Date { day: 1, mon: 1, year: 2000 });
        let dt = fastdate::DateTime::from_timestamp_millis(self.0);
        let micros = if dt >= epoch {
            (dt - epoch).as_micros() as i64
        } else {
            -((epoch - dt).as_micros() as i64)
        };
        micros.encode(buf)
    }
}

impl Decode for Timestamp {
    fn decode(value: PgValue) -> Result<Self, Error> {
        Ok(match value.format() {
            PgValueFormat::Binary => {
                // TIMESTAMP is encoded as the microseconds since the epoch
                let epoch = fastdate::DateTime::from(fastdate::Date {
                    day: 1,
                    mon: 1,
                    year: 2000,
                });
                let us: i64 = Decode::decode(value)?;
                let v = {
                    if us < 0 {
                        epoch - std::time::Duration::from_micros(-us as u64)
                    } else {
                        epoch + std::time::Duration::from_micros(us as u64)
                    }
                };
                Timestamp(v.unix_timestamp_millis())
            }
            PgValueFormat::Text => {
                //2023-11-08 16:38:06.157
                let s = value.as_str()?;
                Timestamp(
                    fastdate::DateTime::from_str(&format!("{}Z", s))
                        .map_err(|e| Error::from(e.to_string()))?
                        .unix_timestamp_millis(),
                )
            }
        })
    }
}
