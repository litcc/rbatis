use std::{collections::HashMap, sync::Arc};

use rbdc::{
    db::{MetaData, Row},
    ext::ustr::UStr,
    Error,
};
use rbs::Value;

use crate::{
    meta_data::MysqlMetaData,
    protocol,
    result_set::{MySqlColumn, MySqlTypeInfo},
    types::Decode,
    value::{MySqlValue, MySqlValueFormat, MySqlValueRef},
};

/// Implementation of [`Row`] for MySQL.
#[derive(Debug)]
pub struct MySqlRow {
    pub row: protocol::Row,
    pub format: MySqlValueFormat,
    pub columns: Arc<Vec<MySqlColumn>>,
    pub column_names: Arc<HashMap<UStr, (usize, MySqlTypeInfo)>>,
}

impl MySqlRow {
    pub fn columns(&self) -> &[MySqlColumn] {
        &self.columns
    }

    pub fn try_get(&self, index: usize) -> Result<MySqlValueRef<'_>, Error> {
        let column: &MySqlColumn = &self.columns[index];
        let value = self.row.get(index as usize);
        Ok(MySqlValueRef {
            format: self.format,
            type_info: column.type_info.clone(),
            value,
        })
    }

    pub fn try_take(&mut self, index: usize) -> Option<MySqlValue> {
        let column: &MySqlColumn = &self.columns[index];
        let value = self.row.take(index)?;
        Some(MySqlValue {
            value: Some(value),
            type_info: column.type_info.clone(),
            format: self.format,
        })
    }
}

impl Row for MySqlRow {
    fn meta_data(&self) -> Box<dyn MetaData> {
        Box::new(MysqlMetaData {
            inner: self.column_names.clone(),
        })
    }

    fn get(&mut self, i: usize) -> Result<Value, Error> {
        match self.try_take(i) {
            None => Ok(Value::Null),
            Some(v) => Value::decode(v),
        }
    }
}
