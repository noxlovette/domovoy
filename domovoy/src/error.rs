use thiserror::Error;

pub(crate) type Res<T> = Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("You are not logged in. Run `domovoy auth`")]
    LoginNeeded,
    #[error("API error: {0}")]
    Api(String),
}
