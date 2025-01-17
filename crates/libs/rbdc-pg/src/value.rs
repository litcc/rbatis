use std::borrow::Cow;
use std::str::from_utf8;

use bytes::Buf;
use rbdc::Error;

use crate::type_info::PgTypeInfo;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum PgValueFormat {
    Text = 0,
    Binary = 1,
}

/// Implementation of [`ValueRef`] for PostgreSQL.
#[derive(Clone)]
pub struct PgValueRef<'r> {
    pub(crate) value: Option<&'r [u8]>,
    pub(crate) type_info: PgTypeInfo,
    pub(crate) format: PgValueFormat,
}

/// Implementation of [`Value`] for PostgreSQL.
#[derive(Clone)]
pub struct PgValue {
    pub(crate) value: Option<Vec<u8>>,
    pub(crate) type_info: PgTypeInfo,
    pub(crate) format: PgValueFormat,
}

impl<'r> PgValueRef<'r> {
    pub(crate) fn get(
        buf: &mut &'r [u8],
        format: PgValueFormat,
        ty: PgTypeInfo,
    ) -> Self {
        let mut element_len = buf.get_i32();

        let element_val = if element_len == -1 {
            element_len = 0;
            None
        } else {
            Some(&buf[..(element_len as usize)])
        };

        buf.advance(element_len as usize);

        PgValueRef { value: element_val, type_info: ty, format }
    }

    pub fn format(&self) -> PgValueFormat {
        self.format
    }

    pub fn as_bytes(&self) -> Result<&'r [u8], Error> {
        match &self.value {
            Some(v) => Ok(v),
            None => Err(Error::from("UnexpectedNullError")),
        }
    }

    pub fn as_str(&self) -> Result<&'r str, Error> {
        Ok(from_utf8(self.as_bytes()?)?)
    }
}

impl PgValue {
    pub fn get(buf: &mut &[u8], format: PgValueFormat, ty: PgTypeInfo) -> Self {
        let mut element_len = buf.get_i32();

        let element_val = if element_len == -1 {
            element_len = 0;
            None
        } else {
            Some(buf[..(element_len as usize)].to_vec())
        };

        buf.advance(element_len as usize);

        PgValue { value: element_val, type_info: ty, format }
    }

    #[inline]
    pub fn as_ref(&self) -> PgValueRef<'_> {
        PgValueRef {
            value: self.value.as_deref(),
            type_info: self.type_info.clone(),
            format: self.format,
        }
    }

    pub fn type_info(&self) -> Cow<'_, PgTypeInfo> {
        Cow::Borrowed(&self.type_info)
    }

    pub fn is_null(&self) -> bool {
        self.value.is_none()
    }

    pub fn format(&self) -> PgValueFormat {
        self.format
    }
    pub fn as_str(&self) -> Result<&str, Error> {
        Ok(from_utf8(self.as_bytes()?)?)
    }

    pub fn as_bytes(&self) -> Result<&[u8], Error> {
        match &self.value {
            Some(v) => Ok(v),
            None => Err(Error::from("UnexpectedNullError")),
        }
    }

    pub fn into_bytes(self) -> Result<Vec<u8>, Error> {
        match self.value {
            Some(v) => Ok(v),
            None => Err(Error::from("UnexpectedNullError")),
        }
    }
}

impl PgValueRef<'_> {
    pub fn to_owned(&self) -> PgValue {
        let value = self.value.map(|value| value.to_vec());
        PgValue { value, format: self.format, type_info: self.type_info.clone() }
    }

    pub fn type_info(&self) -> Cow<'_, PgTypeInfo> {
        Cow::Borrowed(&self.type_info)
    }

    pub fn is_null(&self) -> bool {
        self.value.is_none()
    }
}
