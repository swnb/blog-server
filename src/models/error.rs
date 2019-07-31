use diesel::result::Error as SqlError;

pub fn ignore<T>(_: T) -> Error {
	Error::Other
}

pub enum Error {
	DataBaseError(String),
	ParseError,
	NotFound,
	Other,
}

impl From<SqlError> for Error {
	fn from(sql_error: SqlError) -> Self {
		match sql_error {
			SqlError::DatabaseError(_, reason) => {
				let message = reason.message();
				let details = reason.details().unwrap_or("");
				let reason = format!("error: {} - details: {}", message, details);
				Self::DataBaseError(reason)
			}
			SqlError::NotFound => Self::NotFound,
			_ => Self::Other,
		}
	}
}
