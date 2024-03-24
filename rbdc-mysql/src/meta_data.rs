use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
    ops::Deref,
    sync::Arc,
};

use rbdc::{db::MetaData, ext::ustr::UStr};

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
        return String::new();
    }

    fn column_type(&self, i: usize) -> String {
        for (_, (idx, type_info)) in self.inner.deref() {
            if idx.eq(&i) {
                return format!("{:?}", type_info.r#type);
            }
        }
        return String::new();
    }
}
