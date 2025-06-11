#[derive(Debug)]
pub enum Error {
    AuthFailed,
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
