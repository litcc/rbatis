use rbs::Value;

use crate::ops::AsProxy;
use crate::ops::From;

impl From<bool> for bool {
    fn op_from(arg: bool) -> Self {
        arg
    }
}
impl From<&bool> for bool {
    fn op_from(arg: &bool) -> Self {
        *arg
    }
}
impl From<&&bool> for bool {
    fn op_from(arg: &&bool) -> Self {
        **arg
    }
}

impl From<&Value> for bool {
    fn op_from(arg: &Value) -> Self {
        arg.bool()
    }
}

impl From<&&Value> for bool {
    fn op_from(arg: &&Value) -> Self {
        arg.bool()
    }
}

impl From<Value> for bool {
    fn op_from(arg: Value) -> Self {
        arg.bool()
    }
}
