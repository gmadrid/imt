use clap::{App, AppSettings, Arg, ArgMatches};
use finddups::AddFinddupsSubcommand;
use result::{Error, Result};
use std::env;
use std::ffi::OsString;

static CONFIG_FILE_LOCATION: &'static str = "config";

pub fn parse<'a>() -> Result<ArgMatches<'a>> {
  parse_from(env::args_os())
}

fn parse_from<'a, I, T>(itr: I) -> Result<ArgMatches<'a>>
  where I: IntoIterator<Item = T>,
        T: Into<OsString> {
  parse_cmd_line_from(itr)
}

fn parse_cmd_line_from<'a, I, T>(itr: I) -> Result<ArgMatches<'a>>
  where I: IntoIterator<Item = T>,
        T: Into<OsString> {
  App::new("imt")
  // App configuration
    .about("Collection of image tools in one command")
    .author("George Madrid <gmadrid@gmail.com>")
    .version("0.0.1")
    .setting(AppSettings::SubcommandRequired)

    // App arguments
    .arg(Arg::with_name(CONFIG_FILE_LOCATION)
      .short("C")
      .long(CONFIG_FILE_LOCATION)
      .takes_value(true)
      .help("Location of the config file"))

    // Subcommand arguments
    .add_finddups_subcommand()

    // Process it.
    .get_matches_from_safe(itr)
    .map_err(Error::from)
}
