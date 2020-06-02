pub use self::error::{Error, ErrorKind, Result};
pub use self::object::*;
pub use self::crypto_op::*;
pub use self::time::*;
pub use self::arithmetical::*;
pub use self::internal::*;
pub use self::parameter::{ParamType, ParamTypes, Parameter, Parameters};
pub use optee_utee_macros::{
    ta_close_session, ta_create, ta_destroy, ta_invoke_command, ta_open_session,
};

pub mod trace;
#[macro_use]
mod macros;
mod error;
mod parameter;
pub mod object;
pub mod crypto_op;
pub mod time;
pub mod arithmetical;
pub mod internal;
