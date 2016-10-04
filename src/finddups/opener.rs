use result::Result;
use super::args::{PREVIEW, PRINT, QUIET};

pub trait Opener {
  fn open_group(&self, group: &Vec<&str>) -> Result<()>;
}

mod print {
  use result::Result;
  use super::Opener;

  pub struct PrintOpener {}

  impl Opener for PrintOpener {
    fn open_group(&self, group: &Vec<&str>) -> Result<()> {
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
  use result::Result;
  use super::Opener;

  pub struct NoopOpener {}

  impl Opener for NoopOpener {
    fn open_group(&self, _: &Vec<&str>) -> Result<()> {
      Ok(())
    }
  }

  pub fn new() -> NoopOpener {
    NoopOpener {}
  }
}

mod preview {
  use result::Result;
  use std::process::Command;
  use super::Opener;

  pub struct PreviewOpener {}

  impl Opener for PreviewOpener {
    fn open_group(&self, group: &Vec<&str>) -> Result<()> {
      let mut cmd = Command::new("open");
      for path in group {
        cmd.arg(path);
      }
      Ok(try!(cmd.spawn().map(|_| ())))
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
