//! Error and Result types for the library
use std::convert::From;
use std::error::Error;
use std::fmt::Display;

use rustc_serialize::json::DecoderError;

use http::Error as HttpError;

/// Errors returned by strava API methods
///
// TODO some of these should take other error types.
#[derive(Debug)]
pub enum ApiError {
    /// The given access token has insufficient permission for accessing the requested resource.
    InvalidAccessToken,
    /// Error in the underlying http implementation
    Http(HttpError),
    /// Failed to decode a JSON response from the Strava servers
    InvalidJson(DecoderError)
}

/// A Result type for strava methods
pub type Result<T> = ::std::result::Result<T, ApiError>;

impl Error for ApiError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            ApiError::InvalidJson(ref e) => Some(e),
            ApiError::Http(ref e) => Some(e),
            _ => None
        }
    }

    fn description(&self) -> &str {
        match *self {
            ApiError::InvalidAccessToken => "The token provided was rejected by the server",
            ApiError::Http(ref e) => e.description(),
            ApiError::InvalidJson(ref e) => e.description()
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            ApiError::InvalidAccessToken => write!(f, "ApiError::InvalidAccessToken"),
            ApiError::Http(ref e) => write!(f, "ApiError::Http({})", e),
            ApiError::InvalidJson(ref e) => write!(f, "ApiError::InvalidJson({})", e)
        }
    }
}

impl From<DecoderError> for ApiError {
    fn from(e: DecoderError) -> ApiError {
        ApiError::InvalidJson(e)
    }
}

impl From<HttpError> for ApiError {
    fn from(e: HttpError) -> ApiError {
        ApiError::Http(e)
    }
}
