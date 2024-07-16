pub type MQLResult<T> = Result<T, MQLError>;
pub type MQLError = (i64, String);
pub use crate::enums::*;
pub use crate::runtime::python::PythonRuntime;
pub use crate::schemas::*;
pub use crate::traits::*;
