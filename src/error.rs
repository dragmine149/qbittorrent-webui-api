#[derive(Debug)]
pub enum Error {
    AuthFailed,
    HttpRequestError(reqwest::Error),
    ParseError(url::ParseError),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::HttpRequestError(value)
    }
}

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Self::ParseError(value)
    }
}
