use clap;
use std::io;
use std::result;
use walkdir;

#[derive(Debug)]
pub enum Error {
  Clap(clap::Error),
  IO(io::Error),
  Walkdir(walkdir::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl From<clap::Error> for Error {
  fn from(err: clap::Error) -> Error {
    Error::Clap(err)
  }
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Error {
    Error::IO(err)
  }
}

impl From<walkdir::Error> for Error {
  fn from(err: walkdir::Error) -> Error {
    Error::Walkdir(err)
  }
}
