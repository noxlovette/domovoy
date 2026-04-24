use thiserror::Error;

pub(crate) type Res<T> = Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error while parsing url.")]
    UrlParse(#[from] url::ParseError),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Invalid authentication token.")]
    AuthToken,
}
