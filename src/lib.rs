pub extern crate dark_std;
pub extern crate rbatis_codegen;
extern crate rbatis_macro_driver;
pub extern crate rbdc;

pub use rbatis_macro_driver::{html_sql, py_sql, snake_name, sql};

pub mod plugin;

pub mod rbatis;
#[macro_use]
pub mod utils;
pub mod executor;
#[macro_use]
pub mod crud;
#[macro_use]
pub mod error;
pub mod decode;

pub mod sql;

#[cfg(feature = "option")]
pub mod crud_ref;

pub use async_trait::async_trait;
pub use decode::*;
pub use error::*;
#[cfg(feature = "option")]
pub use paste;
pub use plugin::*;
pub use rbatis::*;
#[cfg(feature = "option")]
pub use rbatis_macro_driver::RefModel;
pub use rbdc_pool_fast::FastPool as DefaultPool;
