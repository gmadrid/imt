use clap::{App, Arg, SubCommand};

static ACTION: &'static str = "action";
static DIR: &'static str = "DIR";
static SUBCOMMAND: &'static str = "finddups";

const PREVIEW: &'static str = "preview";
const PRINT: &'static str = "print";
const QUIET: &'static str = "quiet";
const ACTION_NAMES: [&'static str; 3] = [PREVIEW, PRINT, QUIET];

// #[derive(Debug)]
// pub struct FinddupsArgs {

// }

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
