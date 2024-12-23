use std::fmt::Debug;
use std::sync::Arc;

use crate::statement::PgStatementMetadata;

#[derive(Debug)]
pub struct PgMetaData {
    pub metadata: Arc<PgStatementMetadata>,
}

impl rbdc::db::MetaData for PgMetaData {
    fn column_len(&self) -> usize {
        self.metadata.columns.len()
    }

    fn column_name(&self, i: usize) -> String {
        for (s, idx) in &self.metadata.column_names {
            if idx.eq(&i) {
                return s.to_string();
            }
        }
        String::new()
    }

    fn column_type(&self, i: usize) -> String {
        self.metadata.columns[i].name.to_string()
    }
}
