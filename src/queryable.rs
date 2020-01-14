use log::{error, info, warn};
use rdbc::Connection;
use serde::de;

use crate::decode::rdbc_driver_decoder::decode_result_set;

pub trait Queryable {
    fn query<T>(&mut self, enable_log: bool, sql: &str, arg_array: &[rdbc::Value]) -> Result<T, String> where T: de::DeserializeOwned;
    fn exec(&mut self, enable_log: bool, sql: &str, arg_array: &[rdbc::Value]) -> Result<u64, String>;
}


impl Queryable for Box<dyn Connection> {
    fn query<T>(&mut self, enable_log: bool, sql: &str, arg_array: &[rdbc::Value]) -> Result<T, String> where T: de::DeserializeOwned {
        let create_result = self.create(sql);
        if create_result.is_err() {
            return Result::Err("[rbatis] select fail:".to_string() + format!("{:?}", create_result.err().unwrap()).as_str());
        }
        let mut create_statement = create_result.unwrap();
        let exec_result = create_statement.execute_query(&arg_array);
        if exec_result.is_err() {
            return Result::Err("[rbatis] select fail:".to_string() + format!("{:?}", exec_result.err().unwrap()).as_str());
        }
        let (result, decoded_num) = decode_result_set(exec_result.unwrap().as_mut());
        if enable_log {
            info!("[rbatis] Total: <==  {}", decoded_num.to_string().as_str());
        }
        return result;
    }

    fn exec(&mut self, enable_log: bool, sql: &str, arg_array: &[rdbc::Value]) -> Result<u64, String> {
        let create_result = self.create(sql);
        if create_result.is_err() {
            return Result::Err("[rbatis] exec fail:".to_string() + format!("{:?}", create_result.err().unwrap()).as_str());
        }
        let exec_result = create_result.unwrap().execute_update(&arg_array);
        if exec_result.is_err() {
            return Result::Err("[rbatis] exec fail:".to_string() + format!("{:?}", exec_result.err().unwrap()).as_str());
        }
        let affected_rows = exec_result.unwrap();
        if enable_log {
            info!("[rbatis] Affected: <== {}", affected_rows.to_string().as_str());
        }
        return Result::Ok(affected_rows);
    }
}