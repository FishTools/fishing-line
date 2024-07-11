pub type MQLResult<T> = Result<T, MQLError>;
pub type MQLError = (i64, String);
