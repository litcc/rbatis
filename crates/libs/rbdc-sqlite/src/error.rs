use std::borrow::Cow;
use std::error::Error as StdError;
use std::ffi::CStr;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::{self};
use std::os::raw::c_int;
use std::str::from_utf8_unchecked;

use libsqlite3_sys::sqlite3;
use libsqlite3_sys::sqlite3_errmsg;
use libsqlite3_sys::sqlite3_extended_errcode;

// Error Codes And Messages
// https://www.sqlite.org/c3ref/errcode.html

#[derive(Debug)]
pub struct SqliteError {
    code: c_int,
    message: String,
}

impl SqliteError {
    pub(crate) fn new(handle: *mut sqlite3) -> Self {
        // returns the extended result code even when extended result codes are
        // disabled
        let code: c_int = unsafe { sqlite3_extended_errcode(handle) };

        // return English-language text that describes the error
        let message = unsafe {
            let msg = sqlite3_errmsg(handle);
            debug_assert!(!msg.is_null());

            from_utf8_unchecked(CStr::from_ptr(msg).to_bytes())
        };

        Self { code, message: message.to_owned() }
    }
}

impl Display for SqliteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(&self.message)
    }
}

impl StdError for SqliteError {}

impl SqliteError {
    /// The extended result code.
    #[inline]
    pub fn code(&self) -> Option<Cow<'_, str>> {
        Some(format!("{}", self.code).into())
    }

    #[inline]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[doc(hidden)]
    pub fn as_error(&self) -> &(dyn StdError + Send + Sync + 'static) {
        self
    }

    #[doc(hidden)]
    pub fn as_error_mut(&mut self) -> &mut (dyn StdError + Send + Sync + 'static) {
        self
    }

    #[doc(hidden)]
    pub fn into_error(self: Box<Self>) -> Box<dyn StdError + Send + Sync + 'static> {
        self
    }
}

impl From<SqliteError> for rbdc::Error {
    fn from(e: SqliteError) -> Self {
        Self::from(e.to_string())
    }
}
