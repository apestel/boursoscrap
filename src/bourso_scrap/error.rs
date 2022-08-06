use std::fmt;

#[derive(Debug)]
pub enum BoursoScrapeError {
    HttpError,
    ParseError,
}

impl fmt::Display for BoursoScrapeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HttpError => write!(f, "Http Error"),
            Self::ParseError => write!(f, "Parse Error"),
        }
    }
}

impl From<reqwest::Error> for BoursoScrapeError {
    fn from(_: reqwest::Error) -> Self {
        Self::HttpError
    }
}

impl std::error::Error for BoursoScrapeError {}
