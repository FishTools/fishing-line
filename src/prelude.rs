pub type MQLResult<T> = Result<T, MQLError>;
pub type MQLError = (RuntimeError, String);
pub use crate::connection::python::MT5PythonConnection;
pub use crate::enums::*;
pub use crate::schemas::*;
pub use crate::traits::*;
pub use struct_iterable::Iterable;
