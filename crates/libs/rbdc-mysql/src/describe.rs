use std::convert::identity;

use either::Either;

use crate::result_set::MySqlColumn;
use crate::result_set::MySqlTypeInfo;

/// Provides extended information on a statement.
///
/// Returned from [`Executor::describe`].
///
/// The query macros (e.g., `query!`, `query_as!`, etc.) use the information here to
/// validate output and parameter types; and, generate an anonymous record.
#[derive(Debug)]
#[doc(hidden)]
pub struct Describe {
    pub(crate) columns: Vec<MySqlColumn>,
    pub(crate) parameters: Option<Either<Vec<MySqlTypeInfo>, usize>>,
    pub(crate) nullable: Vec<Option<bool>>,
}

impl Describe {
    /// Gets all columns in this statement.
    pub fn columns(&self) -> &[MySqlColumn] {
        &self.columns
    }

    /// Gets the column information at `index`.
    ///
    /// Panics if `index` is out of bounds.
    pub fn column(&self, index: usize) -> &MySqlColumn {
        &self.columns[index]
    }

    /// Gets the available information for parameters in this statement.
    ///
    /// Some drivers may return more or less than others. As an example1,
    /// **PostgreSQL** will return `Some(Either::Left(_))` with a full list of
    /// type information for each parameter. However, **MSSQL** will return
    /// `None` as there is no information available.
    pub fn parameters(&self) -> Option<Either<&[MySqlTypeInfo], usize>> {
        self.parameters.as_ref().map(|p| match p {
            Either::Left(params) => Either::Left(&**params),
            Either::Right(count) => Either::Right(*count),
        })
    }

    /// Gets whether a column may be `NULL`, if this information is available.
    pub fn nullable(&self, column: usize) -> Option<bool> {
        self.nullable.get(column).copied().and_then(identity)
    }
}
