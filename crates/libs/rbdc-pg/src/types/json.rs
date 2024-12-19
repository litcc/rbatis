use std::io::Write;

use rbdc::json::Json;
use rbdc::Error;
use rbs::Value;

use crate::arguments::PgArgumentBuffer;
use crate::type_info::PgTypeInfo;
use crate::types::decode::Decode;
use crate::types::encode::Encode;
use crate::types::encode::IsNull;
use crate::types::TypeInfo;
use crate::value::PgValue;
use crate::value::PgValueFormat;

impl Encode for Json {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
        let mut bytes = self.0.into_bytes();
        if bytes.is_empty() {
            bytes = "null".to_string().into_bytes();
        }
        // we have a tiny amount of dynamic behavior depending if we are resolved to
        // be JSON instead of JSONB
        buf.patch(|buf, ty: &PgTypeInfo| {
            if *ty == PgTypeInfo::JSON || *ty == PgTypeInfo::JSON_ARRAY {
                buf[0] = b' ';
            }
        });

        // JSONB version (as of 2020-03-20)
        buf.push(1);

        // the JSON data written to the buffer is the same regardless of parameter
        // type
        buf.write_all(&bytes)?;

        Ok(IsNull::No)
    }
}

impl Decode for Json {
    fn decode(value: PgValue) -> Result<Self, Error> {
        let fmt = value.format();
        let type_info = value.type_info;
        let mut buf = value.value.unwrap_or_default();
        if buf.is_empty() {
            return Ok(Json("null".to_string()));
        }
        if fmt == PgValueFormat::Binary && type_info == PgTypeInfo::JSONB {
            assert_eq!(
                buf[0], 1,
                "unsupported JSONB format version {}; please open an issue",
                buf[0]
            );
            buf.remove(0);
        }
        Ok(Self(unsafe { String::from_utf8_unchecked(buf) }))
    }
}

pub fn decode_json(value: PgValue) -> Result<Value, Error> {
    let fmt = value.format();
    let type_info = value.type_info;
    let mut buf = value.value.unwrap_or_default();
    if buf.is_empty() {
        return Ok(Value::Null);
    }
    if fmt == PgValueFormat::Binary && type_info == PgTypeInfo::JSONB {
        assert_eq!(
            buf[0], 1,
            "unsupported JSONB format version {}; please open an issue",
            buf[0]
        );
        buf.remove(0);
    }

    Ok(Value::Ext(
        "Json",
        Box::new(Value::String(unsafe { String::from_utf8_unchecked(buf) })),
    ))
    // serde_json::from_str(&unsafe { String::from_utf8_unchecked(buf) })
    //     .map_err(|e| Error::from(e.to_string()))
}

pub fn encode_json(v: Value, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error> {
    // we have a tiny amount of dynamic behavior depending if we are resolved to be
    // JSON instead of JSONB
    buf.patch(|buf, ty: &PgTypeInfo| {
        if *ty == PgTypeInfo::JSON || *ty == PgTypeInfo::JSON_ARRAY {
            buf[0] = b' ';
        }
    });

    // JSONB version (as of 2020-03-20)
    buf.push(1);

    // the JSON data written to the buffer is the same regardless of parameter type
    buf.write_all(&v.to_string().into_bytes())?;

    Ok(IsNull::No)
}

impl TypeInfo for Json {
    fn type_info(&self) -> PgTypeInfo {
        PgTypeInfo::JSONB
    }
}
