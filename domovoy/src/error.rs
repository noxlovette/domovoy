use thiserror::Error;

pub(crate) type Res<T> = Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {}
