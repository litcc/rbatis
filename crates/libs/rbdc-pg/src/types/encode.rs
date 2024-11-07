use rbdc::Error;
use rbs::Value;

use crate::arguments::PgArgumentBuffer;
use crate::arguments::PgArguments;

pub enum IsNull {
    No,
    Yes,
}

pub trait Encode {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull, Error>;
}

impl PgArguments {
    pub fn from_args(args: Vec<Value>) -> Result<Self, Error> {
        let mut arg = PgArguments {
            types: Vec::with_capacity(args.len()),
            buffer: PgArgumentBuffer::default(),
        };
        for x in args {
            arg.add(x)?;
        }
        Ok(arg)
    }
}
