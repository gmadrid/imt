mod counter;
mod args;
mod opener;

use args::ProvidesDirs;
use clap::ArgMatches;
use crawler::Crawler;
use result::Result;
pub use self::args::{AddFinddupsSubcommand, Args, SUBCOMMAND};
use self::opener::FileInfo;
use sha2::Sha256;
use sha2::digest::Digest;
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use walkdir::DirEntry;

struct FinddupsCrawler {
  file_map: HashMap<String, Vec<FileInfo>>,
  counter: counter::Counter,
  opener: Box<opener::Opener>,
}

impl FinddupsCrawler {
  fn new(opener: Box<opener::Opener>) -> FinddupsCrawler {
    FinddupsCrawler {
      file_map: HashMap::new(),
      counter: counter::Counter::new(),
      opener: opener,
    }
  }
}

impl Crawler for FinddupsCrawler {
  fn should_process_file(&self, entry: &DirEntry) -> bool {
    if entry.metadata().map(|m| m.len() < 1).unwrap_or(false) {
      return false;
    }

    if entry.path().file_stem().map_or(false, |stem| stem == ".DS_Store") {
      return false;
    }

    return true;
  }

  fn process_file_entry(&mut self, entry: &DirEntry) -> Result<()> {
    let path = entry.path();
    let mut f = try!(fs::File::open(path));
    let mut buffer = Vec::new();
    try!(f.read_to_end(&mut buffer));

    let mut hasher = Sha256::new();
    hasher.input(buffer.as_slice());
    let hash = hasher.result_str();
    let md = try!(entry.metadata());
    let create_time = try!(md.created());

    let fname = entry.path().as_os_str();
    let vec = self.file_map.entry(hash).or_insert_with(|| Vec::new());
    try!(self.counter.inc(Some(&fname.to_string_lossy())));
    vec.push(FileInfo {
      filename: fname.to_owned(),
      create_time: create_time,
    });

    Ok(())
  }

  fn done_processing(&mut self) -> Result<()> {
    match self.counter.done(None) {
      Err(err) => println!("{:?}", err),
      _ => {}
    }

    for v in self.file_map.values() {
      if v.len() > 1 {
        match self.opener.open_group(v.clone()) {
          Err(error) => println!("Error while opening: {:?}", error),
          _ => {}
        }
      }
    }
    Ok(())
  }
}

pub fn do_subcommand<'a>(matches: &ArgMatches<'a>) -> Result<()> {
  let args = Args::new(matches);

  let mut crawler = FinddupsCrawler::new(args.opener());
  try!(crawler.process_dirs(args.dirs()));
  Ok(())
}
