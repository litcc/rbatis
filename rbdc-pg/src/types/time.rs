use std::{str::FromStr, time::Duration};

use rbdc::{common::time::Time, Error};

use crate::{
    arguments::PgArgumentBuffer,
    types::{
        decode::Decode,
        encode::{Encode, IsNull},
    },
    value::{PgValue, PgValueFormat},
};

impl Decode for Time {
    fn decode(value: PgValue) -> Result<Self, Error> {
        match value.format() {
            PgValueFormat::Binary => {
                // TIME is encoded as the microseconds since midnight
                let us = i64::decode(value)?;
                //+microseconds
                let t = fastdate::DateTime::from((
                    fastdate::Date {
                        day: 1,
                        mon: 1,
                        year: 2000,
                    },
                    fastdate::Time {
                        nano: 0,
                        sec: 0,
                        minute: 0,
                        hour: 0,
                    },
                ));
                let t = {
                    if us < 0 {
                        t - Duration::from_micros(-us as u64)
                    } else {
                        t + Duration::from_micros(us as u64)
                    }
                };
                Ok(Time(fastdate::Time {
                    nano: t.nano(),
                    sec: t.sec(),
                    minute: t.minute(),
                    hour: t.hour(),
                }))
            }
            PgValueFormat::Text => {
                Ok(Time(fastdate::Time::from_str(value.as_str()?)?))
            }
        }
    }
}

impl Encode for Time {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        // TIME is encoded as the microseconds since midnight
        // microseconds
        let us = self.0.get_micro()
            + self.0.hour as u32 * 60 * 60 * 1000000
            + self.0.minute as u32 * 60 * 1000000
            + self.0.sec as u32 * 1000000;
        us.encode(buf)
    }
}
