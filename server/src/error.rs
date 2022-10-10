
use std::fmt;

use fire_api::error::{ApiError, Error as BasicError, StatusCode};

use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Error {
	Internal(String),
	Request(String)
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		fmt::Debug::fmt(self, f)
	}
}

impl ApiError for Error {
	fn internal<E: BasicError>(error: E) -> Self {
		Self::Internal(error.to_string())
	}

	fn request<E: BasicError>(error: E) -> Self {
		Self::Request(error.to_string())
	}

	fn status_code(&self) -> StatusCode {
		match self {
			Self::Internal(_) => StatusCode::InternalServerError,
			Self::Request(_) => StatusCode::BadRequest
		}
	}
}