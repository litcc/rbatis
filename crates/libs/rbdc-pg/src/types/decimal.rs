use bigdecimal::BigDecimal;
use rbdc::decimal::Decimal;
use rbdc::Error;

use crate::arguments::PgArgumentBuffer;
use crate::types::decode::Decode;
use crate::types::encode::Encode;
use crate::types::encode::IsNull;
use crate::value::PgValue;
use crate::value::PgValueFormat;

impl Encode for Decimal {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        self.0.encode(buf)?;
        Ok(IsNull::No)
    }
}

impl Decode for Decimal {
    fn decode(value: PgValue) -> Result<Self, Error> {
        match value.format() {
            PgValueFormat::Binary => Ok(Self(BigDecimal::decode(value)?)),
            PgValueFormat::Text => Ok(Self(BigDecimal::decode(value)?)),
        }
    }
}
