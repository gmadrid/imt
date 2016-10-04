mod args;

use args::ProvidesDirs;
use clap::ArgMatches;
use result::Result;

pub use self::args::{AddAddextSubcommand, Args, SUBCOMMAND};
use std::ffi::OsStr;
use std::fs::{File, rename};
use std::io::{Read, Seek, SeekFrom};
use walkdir::{DirEntry, WalkDir, WalkDirIterator};

fn should_process(entry: &DirEntry) -> bool {
  let path = entry.path();
  !entry.file_name().to_str().map_or(false, |f| f.starts_with(".")) &&
  (path.is_dir() || path.extension().unwrap_or(OsStr::new("")).len() == 0)
}

fn process_entry(entry: &DirEntry) -> Result<()> {
  let mut file = try!(File::open(entry.path()));
  if try!(is_jpeg(&mut file)) {
    // TODO: make this a flag
    try!(rename(entry.path(), entry.path().with_extension("jpg")));
  }

  Ok(())
}

fn is_jpeg(file: &mut File) -> Result<bool> {
  let head = try!(read_first_two_bytes(file));
  if head != [0xff, 0xd8] {
    return Ok(false);
  }

  let tail = try!(read_last_two_bytes(file));
  if tail != [0xff, 0xd9] {
    return Ok(false);
  }
  Ok(true)
}

fn read_first_two_bytes(file: &mut File) -> Result<[u8; 2]> {
  seek_and_read_two_bytes(file, SeekFrom::Start(0))
}

fn seek_and_read_two_bytes(file: &mut File, location: SeekFrom) -> Result<[u8; 2]> {
  let mut bytes = [0u8; 2];
  try!(file.seek(location));
  try!(file.read_exact(&mut bytes));
  Ok(bytes)
}

fn read_last_two_bytes(file: &mut File) -> Result<[u8; 2]> {
  seek_and_read_two_bytes(file, SeekFrom::End(-2))
}


pub fn do_subcommand<'a>(matches: &ArgMatches<'a>) {
  let args = Args::new(matches);

  for dir in args.dirs() {
    for entry in WalkDir::new(dir).into_iter().filter_entry(|e| should_process(e)) {
      let entry = entry.unwrap();
      if !entry.file_type().is_dir() {
        match process_entry(&entry) {
          Err(err) => println!("Error: {:?}", err),
          Ok(_) => {}
        }
      }
    }
  }

}
