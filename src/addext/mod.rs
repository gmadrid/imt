mod args;

use args::ProvidesDirs;
use clap::ArgMatches;
use result::Result;

pub use self::args::{AddAddextSubcommand, Args, SUBCOMMAND};
use std::ffi::OsStr;
use walkdir::{DirEntry, WalkDir, WalkDirIterator};

fn should_process(entry: &DirEntry) -> bool {
  let path = entry.path();
  !entry.file_name().to_str().map_or(false, |f| f.starts_with(".")) &&
  (path.is_dir() || path.extension().unwrap_or(OsStr::new("")).len() == 0)
}

fn process_entry(entry: &DirEntry) -> Result<()> {
  println!("{}", entry.path().display());
  Ok(())
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
