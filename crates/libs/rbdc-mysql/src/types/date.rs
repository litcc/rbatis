use std::str::FromStr;

use byteorder::ByteOrder;
use byteorder::LittleEndian;
use rbdc::date::Date;
use rbdc::Error;

use crate::types::Decode;
use crate::types::Encode;
use crate::value::MySqlValue;
use crate::value::MySqlValueFormat;

impl Encode for Date {
    fn encode(self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        buf.push(4);
        //u16:  MySQL supports years from 1000 - 9999
        let year = &(self.0.year as u16).to_le_bytes();
        buf.extend_from_slice(year);
        buf.push(self.0.mon);
        buf.push(self.0.day);
        Ok(4)
    }
}

impl Decode for Date {
    fn decode(value: MySqlValue) -> Result<Self, Error> {
        Ok(Date(match value.format() {
            MySqlValueFormat::Text => fastdate::Date::from_str(value.as_str()?)
                .map_err(|e| Error::from(e.to_string()))?,
            MySqlValueFormat::Binary => {
                let buf = value.as_bytes()?;
                //let len = buf[0];
                decode_date_buf(&buf[1..])?
            }
        }))
    }
}

pub fn decode_date_buf(buf: &[u8]) -> Result<fastdate::Date, Error> {
    if buf.is_empty() {
        // zero buffer means a zero date (null)
        return Ok(fastdate::Date { day: 0, mon: 0, year: 0 });
    }
    Ok(fastdate::Date {
        day: buf[3],
        mon: buf[2],
        year: LittleEndian::read_u16(buf) as i32,
    })
}
