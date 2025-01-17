use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::{self};

use crate::protocol::response::ErrPacket;

/// An error returned from the MySQL database.
pub struct MySqlDatabaseError(pub(super) ErrPacket);

impl MySqlDatabaseError {
    /// The [SQLSTATE](https://dev.mysql.com/doc/refman/8.0/en/server-error-reference.html) code for this error.
    pub fn code(&self) -> Option<&str> {
        self.0.sql_state.as_deref()
    }

    /// The [number](https://dev.mysql.com/doc/refman/8.0/en/server-error-reference.html)
    /// for this error.
    ///
    /// MySQL tends to use SQLSTATE as a general error category, and the error number
    /// as a more granular indication of the error.
    pub fn number(&self) -> u16 {
        self.0.error_code
    }

    /// The human-readable error message.
    pub fn message(&self) -> &str {
        &self.0.error_message
    }
}

impl Debug for MySqlDatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("MySqlDatabaseError")
            .field("code", &self.code())
            .field("number", &self.number())
            .field("message", &self.message())
            .finish()
    }
}

impl Display for MySqlDatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(code) = &self.code() {
            write!(f, "{} ({}): {}", self.number(), code, self.message())
        } else {
            write!(f, "{}: {}", self.number(), self.message())
        }
    }
}

impl Error for MySqlDatabaseError {}

impl MySqlDatabaseError {
    #[doc(hidden)]
    fn as_error(&self) -> &(dyn Error + Send + Sync + 'static) {
        self
    }

    #[doc(hidden)]
    fn as_error_mut(&mut self) -> &mut (dyn Error + Send + Sync + 'static) {
        self
    }

    #[doc(hidden)]
    fn into_error(self: Box<Self>) -> Box<dyn Error + Send + Sync + 'static> {
        self
    }
}

impl From<MySqlDatabaseError> for rbdc::Error {
    fn from(arg: MySqlDatabaseError) -> Self {
        rbdc::Error::from(arg.to_string())
    }
}
