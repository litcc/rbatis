use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::ops::Deref;
use std::sync::Arc;

use rbdc::db::MetaData;
use rbdc::ext::ustr::UStr;

use crate::result_set::MySqlTypeInfo;

pub struct MysqlMetaData {
    pub inner: Arc<HashMap<UStr, (usize, MySqlTypeInfo)>>,
}
impl Debug for MysqlMetaData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl MetaData for MysqlMetaData {
    fn column_len(&self) -> usize {
        self.inner.len()
    }

    fn column_name(&self, i: usize) -> String {
        for (s, (idx, _)) in self.inner.deref() {
            if idx.eq(&i) {
                return s.to_string();
            }
        }
        String::new()
    }

    fn column_type(&self, i: usize) -> String {
        for (idx, type_info) in self.inner.deref().values() {
            if idx.eq(&i) {
                return format!("{:?}", type_info.r#type);
            }
        }
        String::new()
    }
}
