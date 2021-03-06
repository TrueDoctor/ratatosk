//! Collection of frontend errors.
//! These can result from network errors, other JavaScript errors or concurrency errors.
//!
//! All errors implement (Display)[https://doc.rust-lang.org/std/fmt/trait.Display.html].
//!
//! # Examples
//!
//! ```should_panic
//! use rask_wasm::error::ClientError;
//!
//! # fn main() -> Result<(), ClientError> {
//! return Err(ClientError::EngineError(format!("EngineError")));
//! # }
//! ```

use std::fmt;

#[derive(Debug)]
pub enum ClientError {
    WebGlError(String),
    ResourceError(String),
    EngineError(String),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::ResourceError(e)
            | ClientError::WebGlError(e)
            | ClientError::EngineError(e) => write!(f, "{}", e),
        }
    }
}

/// Derive from implementations.
///
/// # Example
///
/// ```
/// derive_from!(rask_engine::error::EngineError, EngineError);
/// ```
macro_rules! derive_from {
    ($type:ty, $kind:ident) => {
        impl From<$type> for ClientError {
            fn from(error: $type) -> Self {
                ClientError::$kind(format!("{}", error))
            }
        }
    };
}

derive_from!(rask_engine::error::EngineError, EngineError);
derive_from!(crate::graphics::webgl::WebGl2Error, WebGlError);
