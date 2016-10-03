use clap;
use std::io;
use std::result;

#[derive(Debug)]
pub enum Error {
  Clap(clap::Error),
  IO(io::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl From<clap::Error> for Error {
  fn from(err: clap::Error) -> Error {
    Error::Clap(err)
  }
}
