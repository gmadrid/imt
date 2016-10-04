extern crate clap;
extern crate sha2;
extern crate term_size;
extern crate walkdir;

mod addext;
mod args;
mod finddups;
mod result;

use result::{Error, Result};

fn real_main() -> Result<()> {
  let matches = try!(args::parse());
  match matches.subcommand() {
    (addext::SUBCOMMAND, Some(sub_matches)) => addext::do_subcommand(sub_matches),
    (finddups::SUBCOMMAND, Some(sub_matches)) => finddups::do_subcommand(sub_matches),
    _ => {} // no-op. clap should ensure that there is always a subcommand.
  }

  Ok(())
}

fn main() {
  // A shell that calls a "real main" function and reports errors.
  // A convenience so that I can try!() inside the "main" function.
  match real_main() {
    Ok(_) => (),
    Err(err) => {
      match err {
        // Clap gets special attention. ('-h' for example is better handled by clap::Error::exit())
        Error::Clap(ce) => clap::Error::exit(&ce),
        _ => println!("{:?}", err),
      }
    }
  }
}
