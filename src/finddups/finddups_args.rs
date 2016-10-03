use clap::{App, SubCommand};

static SUBCOMMAND: &'static str = "finddups";

// #[derive(Debug)]
// pub struct FinddupsArgs {

// }

pub trait AddFinddupsSubcommand<'a, 'b> {
  fn add_finddups_subcommand(self) -> App<'a, 'b>;
}

impl<'a, 'b> AddFinddupsSubcommand<'a, 'b> for App<'a, 'b> {
  fn add_finddups_subcommand(self) -> App<'a, 'b> {
    self.subcommand(SubCommand::with_name(SUBCOMMAND))
      .about("find exact duplicates across a list of directories")
  }
}
