use clap::{App, AppSettings, Arg, ArgMatches};
use finddups;
use result::{Error, Result};
use std::env;
use std::ffi::OsString;

static CONFIG_FILE_LOCATION: &'static str = "config";

#[derive(Debug)]
pub struct Args<'a> {
  matches: ArgMatches<'a>,
}

impl<'a> Args<'a> {
  pub fn parse() -> Result<Args<'a>> {
    Args::parse_from(env::args_os())
  }

  fn parse_from<I, T>(itr: I) -> Result<Args<'a>>
    where I: IntoIterator<Item = T>,
          T: Into<OsString> {
    let matches = try!(parse_cmd_line_from(itr));
    Ok(Args { matches: matches })
  }
}

fn parse_cmd_line_from<'a, I, T>(itr: I) -> Result<ArgMatches<'a>>
  where I: IntoIterator<Item = T>,
        T: Into<OsString> {
  let builder = App::new("imt")
    .setting(AppSettings::SubcommandRequired)
    .version("0.0.1")
    .arg(Arg::with_name(CONFIG_FILE_LOCATION)
      .short("C")
      .long(CONFIG_FILE_LOCATION)
      .takes_value(true)
      .help("Location of the config file"));

  let builder = finddups::add_subcommand(builder);

  builder.get_matches_from_safe(itr)
    .map_err(Error::from)
}
