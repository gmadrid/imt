use clap::{App, SubCommand};

static SUBCOMMAND: &'static str = "finddups";

#[derive(Debug)]
pub struct FinddupsArgs {

}

pub fn add_subcommand<'a, 'b>(builder: App<'a, 'b>) -> App<'a, 'b> {
  builder.subcommand(SubCommand::with_name(SUBCOMMAND))
}
