/// Error that can occur.
///
/// This enum encapsulates various types of errors, including authentication failures,
/// HTTP request errors, URL parsing errors, and JSON serialization/deserialization errors.
#[derive(Debug)]
pub enum Error {
    AuthFailed(String),
    InvalidResponse(String),
    HttpRequestError(reqwest::Error),
    UrlParseError(url::ParseError),
    SerdJsonError(serde_json::Error),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::HttpRequestError(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Self::UrlParseError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdJsonError(err)
    }
}
