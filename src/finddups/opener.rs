use std::io;
use super::finddups_args::{PREVIEW, PRINT, QUIET};

pub trait Opener {
  fn open_group(&self, group: &Vec<&str>) -> io::Result<()>;
}

mod print {
  use std::io;
  use super::Opener;

  pub struct PrintOpener {}

  impl Opener for PrintOpener {
    fn open_group(&self, group: &Vec<&str>) -> io::Result<()> {
      println!("GROUP:");
      for path in group {
        println!("    {}", path);
      }
      Ok(())
    }
  }

  pub fn new() -> PrintOpener {
    PrintOpener {}
  }
}

mod noop {
  use std::io;
  use super::Opener;

  pub struct NoopOpener {}

  impl Opener for NoopOpener {
    fn open_group(&self, _: &Vec<&str>) -> io::Result<()> {
      Ok(())
    }
  }

  pub fn new() -> NoopOpener {
    NoopOpener {}
  }
}

mod preview {
  use std::io;
  use std::process::Command;
  use super::Opener;

  pub struct PreviewOpener {}

  impl Opener for PreviewOpener {
    fn open_group(&self, group: &Vec<&str>) -> io::Result<()> {
      let mut cmd = Command::new("open");
      for path in group {
        cmd.arg(path);
      }
      cmd.spawn().map(|_| ())
    }
  }

  pub fn new() -> PreviewOpener {
    PreviewOpener {}
  }
}

pub fn opener_for_option(option: &str) -> Box<Opener> {
  match option {
    PREVIEW => Box::new(preview::new()),
    QUIET => Box::new(noop::new()),
    PRINT | _ => Box::new(print::new()),
  }
}
