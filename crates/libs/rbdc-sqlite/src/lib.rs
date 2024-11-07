//! **SQLite** database driver.

// SQLite is a C library. All interactions require FFI which is unsafe.
// All unsafe blocks should have comments pointing to SQLite docs and ensuring that
// we maintain invariants.
#![allow(unsafe_code)]
#![allow(dead_code)]

pub use arguments::SqliteArgumentValue;
pub use arguments::SqliteArguments;
pub use column::SqliteColumn;
pub use connection::LockedSqliteHandle;
pub use connection::SqliteConnection;
pub use database::Sqlite;
pub use error::SqliteError;
pub use options::SqliteAutoVacuum;
pub use options::SqliteConnectOptions;
pub use options::SqliteJournalMode;
pub use options::SqliteLockingMode;
pub use options::SqliteSynchronous;
pub use query_result::SqliteQueryResult;
pub use row::SqliteRow;
pub use statement::SqliteStatement;
pub use type_info::SqliteTypeInfo;
pub use value::SqliteValue;
pub use value::SqliteValueRef;

pub mod arguments;
pub mod column;
pub mod connection;
pub mod database;
pub mod decode;
pub mod driver;
pub mod encode;
pub mod error;
pub mod options;
pub mod query;
pub mod query_result;
pub mod row;
pub mod statement;
pub mod type_info;
pub mod types;
pub mod value;

pub use driver::SqliteDriver;
pub use driver::SqliteDriver as Driver;
