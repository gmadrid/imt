use addext::AddAddextSubcommand;
use clap::{App, AppSettings, Arg, ArgMatches};
use finddups::AddFinddupsSubcommand;
use result::{Error, Result};
use std::env;
use std::ffi::OsString;
use std::path::Path;

static CONFIG_FILE_LOCATION: &'static str = "config";
pub static DIR: &'static str = "DIR";

pub trait HasMatches {
  fn matches(&self) -> &ArgMatches;
}

pub trait ProvidesDirs: HasMatches {
  fn dirs(&self) -> Vec<&str> {
    self.matches().values_of(DIR).map_or(vec![], |values| {
      values.filter(|dir| {
          let path = Path::new(dir);
          path.exists() && path.is_dir()
        })
        .collect()
    })
  }
}

pub fn parse<'a>() -> Result<ArgMatches<'a>> {
  parse_from(env::args_os())
}

fn parse_from<'a, I, T>(itr: I) -> Result<ArgMatches<'a>>
  where I: IntoIterator<Item = T>,
        T: Into<OsString> {
  App::new("imt")
  // App configuration
    .about("Collection of image tools in one command")
    .author("George Madrid <gmadrid@gmail.com>")
    .version("0.0.1")
    .setting(AppSettings::StrictUtf8)
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .setting(AppSettings::UnifiedHelpMessage)
    .setting(AppSettings::VersionlessSubcommands)

    // App arguments
    .arg(Arg::with_name(CONFIG_FILE_LOCATION)
      .short("C")
      .long(CONFIG_FILE_LOCATION)
      .takes_value(true)
      .help("Location of the config file"))

    // Subcommand arguments
    .add_addext_subcommand()
    .add_finddups_subcommand()

    // Process it.
    .get_matches_from_safe(itr)
    .map_err(Error::from)
}
