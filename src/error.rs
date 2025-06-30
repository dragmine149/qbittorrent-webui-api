/// Error that can occur.
///
/// This enum encapsulates various types of errors, including authentication failures,
/// HTTP request errors, URL parsing errors, and JSON serialization/deserialization errors.
#[derive(Debug)]
pub enum Error {
    AuthFailed,
    InvalidResponse(String),
    HttpRequestError(reqwest::Error),
    UrlParseError(url::ParseError),
    SerdJsonError(serde_json::Error),
    CookieError(cookie_store::CookieError),
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

impl From<cookie_store::CookieError> for Error {
    fn from(err: cookie_store::CookieError) -> Self {
        Self::CookieError(err)
    }
}
