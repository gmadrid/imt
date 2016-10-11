use result::Result;
use std::ffi::OsString;
use std::time::SystemTime;
use super::args::{DELETE, PREVIEW, PRINT, QUIET};

#[derive(Clone)]
pub struct FileInfo {
  pub filename: OsString,
  pub create_time: SystemTime,
}

pub trait Opener {
  fn open_group(&self, group: Vec<FileInfo>) -> Result<()>;
}

mod print {
  use result::Result;
  use super::{FileInfo, Opener};

  pub struct PrintOpener {}

  impl Opener for PrintOpener {
    fn open_group(&self, group: Vec<FileInfo>) -> Result<()> {
      println!("GROUP:");
      for fi in group {
        println!("    {}", fi.filename.to_string_lossy());
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
  use super::{FileInfo, Opener};

  pub struct NoopOpener {}

  impl Opener for NoopOpener {
    fn open_group(&self, _: Vec<FileInfo>) -> Result<()> {
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
  use super::{FileInfo, Opener};

  pub struct PreviewOpener {
    report_only: bool,
  }

  impl Opener for PreviewOpener {
    fn open_group(&self, group: Vec<FileInfo>) -> Result<()> {
      if self.report_only {
        Ok(())
      } else {
        let mut cmd = Command::new("open");
        for fi in group {
          cmd.arg(&fi.filename);
        }
        Ok(try!(cmd.spawn().map(|_| ())))
      }
    }
  }

  pub fn new(report_only: bool) -> PreviewOpener {
    PreviewOpener { report_only: report_only }
  }
}

mod delete {
  use result::Result;
  use std::fs;
  use super::{FileInfo, Opener};

  pub struct DeleteOpener {
    report_only: bool,
  }

  impl Opener for DeleteOpener {
    fn open_group(&self, mut group: Vec<FileInfo>) -> Result<()> {
      // Check the length to ensure that we can unwrap the first element.
      if group.len() > 1 {
        group.sort_by_key(|fi| fi.create_time);
        let mut i = group.iter();
        println!("Keeping: {}", i.next().unwrap().filename.to_string_lossy());
        for fi in i {
          println!("  Deleting: {}", fi.filename.to_string_lossy());
          if !self.report_only {
            try!(fs::remove_file(&fi.filename));
          }
        }
      }
      Ok(())
    }
  }

  pub fn new(report_only: bool) -> DeleteOpener {
    DeleteOpener { report_only: report_only }
  }
}

pub fn opener_for_option(option: &str, report_only: bool) -> Box<Opener> {
  match option {
    PREVIEW => Box::new(preview::new(report_only)),
    QUIET => Box::new(noop::new()),
    DELETE => Box::new(delete::new(report_only)),
    PRINT | _ => Box::new(print::new()),
  }
}
