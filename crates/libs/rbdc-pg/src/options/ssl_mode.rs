use std::str::FromStr;

use rbdc::Error;

/// Options for controlling the level of protection provided for PostgreSQL SSL
/// connections.
///
/// It is used by the [`ssl_mode`](super::PgConnectOptions::ssl_mode) method.
#[derive(Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[derive(Default)]
pub enum PgSslMode {
    /// Only try a non-SSL connection.
    #[default]
    Disable,

    /// First try a non-SSL connection; if that fails, try an SSL connection.
    Allow,

    /// First try an SSL connection; if that fails, try a non-SSL connection.
    Prefer,

    /// Only try an SSL connection. If a root CA file is present, verify the
    /// connection in the same way as if `VerifyCa` was specified.
    Require,

    /// Only try an SSL connection, and verify that the server certificate is issued
    /// by a trusted certificate authority (CA).
    VerifyCa,

    /// Only try an SSL connection; verify that the server certificate is issued by
    /// a trusted CA and that the requested server host name matches that in the
    /// certificate.
    VerifyFull,
}

impl FromStr for PgSslMode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(match &*s.to_ascii_lowercase() {
            "disable" => PgSslMode::Disable,
            "allow" => PgSslMode::Allow,
            "prefer" => PgSslMode::Prefer,
            "require" => PgSslMode::Require,
            "verify-ca" => PgSslMode::VerifyCa,
            "verify-full" => PgSslMode::VerifyFull,

            _ => {
                return Err(Error::from(format!(
                    "unknown value {:?} for `ssl_mode`",
                    s
                )));
            }
        })
    }
}
