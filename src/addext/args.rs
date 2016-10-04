use args::{DIR, HasMatches, ProvidesDirs};
use clap::{App, Arg, ArgMatches, SubCommand};

pub const SUBCOMMAND: &'static str = "addext";

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
}

pub trait AddAddextSubcommand<'a, 'b> {
  fn add_addext_subcommand(self) -> App<'a, 'b>;
}

impl<'a, 'b> AddAddextSubcommand<'a, 'b> for App<'a, 'b> {
  fn add_addext_subcommand(self) -> App<'a, 'b> {
    self.subcommand(SubCommand::with_name(SUBCOMMAND)
      .about("add extensions to files when they are missing")
      .arg(Arg::with_name(DIR)
        .help("Directories to crawl")
        .index(1)
        .multiple(true)
        .required(true)))
  }
}
