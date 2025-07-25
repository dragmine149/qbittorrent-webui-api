/// Error that can occur.
///
/// This enum encapsulates various types of errors, including authentication failures,
/// HTTP request errors, URL parsing errors, and JSON serialization/deserialization errors.
#[derive(Debug)]
pub enum Error {
    AuthFailed(String),
    InvalidResponse(String),
    InvalidRequest(String),
    ReqwestError(reqwest::Error),
    UrlParseError(url::ParseError),
    SerdeJsonError(serde_json::Error),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::ReqwestError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJsonError(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Self::UrlParseError(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl std::error::Error for Error {}
