use std::{
    fmt::{Debug, Display, Formatter},
    str::FromStr,
};

use rbs::Value;

use crate::{DateTime, Error};

#[derive(serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(rename = "Time")]
pub struct Time(pub fastdate::Time);

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Time({})", self.0)
    }
}

impl From<Time> for Value {
    fn from(arg: Time) -> Self {
        Value::Ext("Time", Box::new(Value::String(arg.0.to_string())))
    }
}

impl FromStr for Time {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Time(fastdate::Time::from_str(s)?))
    }
}

impl From<Time> for fastdate::Time {
    fn from(value: Time) -> Self {
        value.0
    }
}

impl From<DateTime> for Time {
    fn from(value: DateTime) -> Self {
        Self(fastdate::Time::from(value.0))
    }
}

impl Default for Time {
    fn default() -> Self {
        Time(fastdate::Time {
            nano: 0,
            sec: 0,
            minute: 0,
            hour: 0,
        })
    }
}
