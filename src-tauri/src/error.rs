//! This is the main (and only for now) application `Error` type.
//! It's using `thiserror` as it reduces boilerplate error code while providing rich error typing.
//!
//! Notes:
//!   - The strategy to start with one `Error` type for the whole application and then segregate as needed.
//!   - Since everything is typed from the start, renaming and refactoring become relatively trivial.
//!   - By best practices, `anyhow` is not used in application code, but can be used in unit or integration tests (will be in `dev_dependencies` when used)
//!

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Fail to get Ctx")]
    CtxFail,

    #[error("Value not of type '{0}'")]
    XValueNotOfType(&'static str),

    #[error("Property '{0}' not found")]
    XPropertyNotFound(String),

    #[error("Faile to create.  Cause: {0}")]
    StoreFailToCreate(String),

    #[error(transparent)]
    Surreal(#[from] surrealdb::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Inconsistent state: {0}")]
    InvalidState(String),
}
