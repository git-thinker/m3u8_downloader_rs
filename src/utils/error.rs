use std::fmt;

#[derive(Debug)]
pub enum M3u8Error {
    HTTPError(reqwest::Error),
    HTTPCode(reqwest::StatusCode),
    Other,
}

// Allow the use of "{}" format specifier
impl fmt::Display for M3u8Error{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        match self {
            M3u8Error::HTTPError(e) => {
                write!(f, "HTTPError by reqwest: {}", e.to_string())
            },
            M3u8Error::HTTPCode(s) => {
                write!(f, "HTTP request reply with Code: {}", s)
            }
            M3u8Error::Other => {
                write!(f, "Unexpected Error, no info provided.")
            }
        }
    }
}

// implemented for error conversion
impl From<reqwest::Error> for M3u8Error {
    fn from(cause: reqwest::Error) -> M3u8Error {
        M3u8Error::HTTPError(cause)
    }
}