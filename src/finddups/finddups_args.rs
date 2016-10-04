use clap::{App, Arg, ArgMatches, SubCommand};
use std::path::Path;
use super::opener::{self, Opener};

static ACTION: &'static str = "action";
static DIR: &'static str = "DIR";
pub const SUBCOMMAND: &'static str = "finddups";

pub const PREVIEW: &'static str = "preview";
pub const PRINT: &'static str = "print";
pub const QUIET: &'static str = "quiet";
const ACTION_NAMES: [&'static str; 3] = [PREVIEW, PRINT, QUIET];

#[derive(Debug)]
pub struct Args<'a> {
  matches: &'a ArgMatches<'a>,
}

impl<'a> Args<'a> {
  pub fn new<'b>(matches: &'b ArgMatches) -> Args<'b> {
    Args { matches: matches }
  }

  pub fn dirs(&self) -> Vec<&str> {
    let values: Option<Vec<&str>> = self.matches.values_of(DIR).map(|x| x.collect());
    values.map_or_else(|| Vec::new(), |v| {
      v.into_iter()
        .filter(|dir| {
          let path = Path::new(dir);
          // TODO: print a warning when a path is excluded.
          return path.exists() && path.is_dir();
        })
        .collect()
    })
  }

  pub fn opener(&self) -> Box<Opener> {
    // default value guarantees that this will unwrap.
    let action = self.matches.value_of(ACTION).unwrap();
    opener::opener_for_option(action)
  }
}

pub trait AddFinddupsSubcommand<'a, 'b> {
  fn add_finddups_subcommand(self) -> App<'a, 'b>;
}

impl<'a, 'b> AddFinddupsSubcommand<'a, 'b> for App<'a, 'b> {
  fn add_finddups_subcommand(self) -> App<'a, 'b> {
    self.subcommand(SubCommand::with_name(SUBCOMMAND)
      .about("find exact duplicates across a list of directories")
      .arg(Arg::with_name(DIR)
        .help("Directories to crawl")
        .index(1)
        .multiple(true)
        .required(true))
      .arg(Arg::with_name(ACTION)
        .long(ACTION)
        .help("The action to take with matching groups")
        .possible_values(&ACTION_NAMES)
        .default_value(PRINT)))
  }
}
