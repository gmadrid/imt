mod args;

use args::ProvidesDirs;
use clap::ArgMatches;
use crawler::Crawler;
use result::Result;

pub use self::args::{AddAddextSubcommand, Args, SUBCOMMAND};
use std::ffi::OsStr;
use std::fs::{File, rename};
use std::io::{Read, Seek, SeekFrom};
use walkdir::DirEntry;

#[derive(Debug)]
struct AddextCrawler {
  rename: bool,
}

impl AddextCrawler {
  fn new(rename: bool) -> AddextCrawler {
    AddextCrawler { rename: rename }
  }
}

impl Crawler for AddextCrawler {
  fn should_process_file(&self, entry: &DirEntry) -> bool {
    // Skip directories that start with ".". ("Hidden" files, also .DS_Store)
    !entry.file_name().to_str().map_or(false, |f| f.starts_with(".")) &&
    // Also skip files that already have an extension.
    entry.path().extension().unwrap_or(OsStr::new("")).len() == 0
  }

  fn process_file_entry(&mut self, entry: &DirEntry) -> Result<()> {
    let mut file = try!(File::open(entry.path()));
    if try!(is_jpeg(&mut file)) {
      if self.rename {
        try!(rename(entry.path(), entry.path().with_extension("jpg")));
        println!("Renamed: {}", entry.path().display());
      } else {
        println!("{}", entry.path().display());
      }
    }

    Ok(())
  }
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

pub fn do_subcommand<'a>(matches: &ArgMatches<'a>) -> Result<()> {
  let args = Args::new(matches);

  let mut crawler = AddextCrawler::new(args.rename());
  try!(crawler.process_dirs(args.dirs()));
  Ok(())
}
