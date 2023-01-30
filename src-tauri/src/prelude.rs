//! Key default types for this application designed to be imported in most crate modules.
//!
//! Notes:
//!   - The best practice is to have a narrow crate prelude to normalize the key types throughtout the application code.
//!   - We keep this as small as possible, and try to limit generic name besides `Result` and `Error` (which is re-exported from this module)
//!   - The `f!` macro alias of `format!` (personal preference)

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

/// Generic Wrapper typle struct for newtype pattern, mostly for external types to From/TryFrom conversions
pub struct W<T>(pub T);

pub use std::format as f;
