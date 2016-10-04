mod counter;
mod args;
mod opener;

use args::ProvidesDirs;
use clap::ArgMatches;

pub use self::args::{AddFinddupsSubcommand, Args, SUBCOMMAND};
use sha2::Sha256;
use sha2::digest::Digest;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use walkdir::{DirEntry, WalkDir, WalkDirIterator};

struct FileInfo {
  path: String,
}

fn should_skip(entry: &DirEntry) -> bool {
  if let Ok(metadata) = entry.metadata() {
    if metadata.len() < 1 {
      return true;
    }
  } else {
    return true;
  }

  if let Some(stem) = entry.path().file_stem() {
    if stem == ".DS_Store" {
      return true;
    }
  }

  false
}

fn process_file_entry(entry: &DirEntry,
                      file_map: &mut HashMap<String, Vec<FileInfo>>,
                      counter: &mut counter::Counter)
                      -> io::Result<()> {

  let path = entry.path();
  let mut f = try!(fs::File::open(path));
  let mut buffer = Vec::new();
  try!(f.read_to_end(&mut buffer));

  let mut hasher = Sha256::new();
  hasher.input(buffer.as_slice());
  let hash = hasher.result_str();

  let fname = entry.path().to_str().unwrap().to_string();
  let info = FileInfo { path: fname.clone() };
  let vec = file_map.entry(hash).or_insert_with(|| Vec::new());
  try!(counter.inc(Some(&fname)));
  vec.push(info);

  Ok(())
}

pub fn do_subcommand<'a>(matches: &ArgMatches<'a>) {
  let args = Args::new(matches);
  let mut counter = counter::Counter::new();
  let mut map: HashMap<String, Vec<FileInfo>> = HashMap::new();

  for dir in args.dirs() {
    for entry in WalkDir::new(dir)
      .into_iter()
      .filter_entry(|e| !should_skip(e)) {
      let entry = entry.unwrap();
      if entry.file_type().is_file() {
        match process_file_entry(&entry, &mut map, &mut counter) {
          Err(err) => println!("{:?}", err),
          _ => {}
        }
      }
    }
  }
  match counter.done(None) {
    Err(err) => println!("{:?}", err),
    _ => {}
  }

  let opener = args.opener();
  for v in map.values() {
    if v.len() > 1 {
      opener.open_group(&v.iter().map(|fi| fi.path.as_str()).collect()).unwrap();
    }
  }
}
