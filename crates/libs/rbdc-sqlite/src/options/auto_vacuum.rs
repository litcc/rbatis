use std::str::FromStr;

use rbdc::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum SqliteAutoVacuum {
    #[default]
    None,
    Full,
    Incremental,
}

impl SqliteAutoVacuum {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            SqliteAutoVacuum::None => "NONE",
            SqliteAutoVacuum::Full => "FULL",
            SqliteAutoVacuum::Incremental => "INCREMENTAL",
        }
    }
}

impl FromStr for SqliteAutoVacuum {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(match &*s.to_ascii_lowercase() {
            "none" => SqliteAutoVacuum::None,
            "full" => SqliteAutoVacuum::Full,
            "incremental" => SqliteAutoVacuum::Incremental,

            _ => {
                return Err(Error::from(format!(
                    "Configure unknown value {:?} for `auto_vacuum`",
                    s
                )));
            }
        })
    }
}
