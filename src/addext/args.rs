use args::{DIR, HasMatches, ProvidesDirs};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

pub const SUBCOMMAND: &'static str = "addext";
pub const MOVE: &'static str = "rename";

#[derive(Debug)]
pub struct Args<'a> {
  matches: &'a ArgMatches<'a>,
}

impl<'a> HasMatches for Args<'a> {
  fn matches(&self) -> &ArgMatches {
    self.matches
  }
}

impl<'a> ProvidesDirs for Args<'a> {}

impl<'a> Args<'a> {
  pub fn new<'b>(matches: &'b ArgMatches) -> Args<'b> {
    Args { matches: matches }
  }

  pub fn rename(&self) -> bool {
    self.matches.is_present(MOVE)
  }
}

pub trait AddAddextSubcommand<'a, 'b> {
  fn add_addext_subcommand(self) -> App<'a, 'b>;
}

impl<'a, 'b> AddAddextSubcommand<'a, 'b> for App<'a, 'b> {
  fn add_addext_subcommand(self) -> App<'a, 'b> {
    self.subcommand(SubCommand::with_name(SUBCOMMAND)
      .about("add extensions to files when they are missing")
      .setting(AppSettings::UnifiedHelpMessage)
      .arg(Arg::with_name(DIR)
        .help("Directories to crawl")
        .index(1)
        .multiple(true)
        .required(true))
      .arg(Arg::with_name(MOVE)
        .help("Rename file with extension")
        .long(MOVE)))
  }
}
