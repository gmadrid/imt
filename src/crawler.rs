// A thin shim around WalkDir.

use result::{Error, Result};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub trait Crawler {
  // Filter function. Return true to call process_file_entry with this entry.
  fn should_process_file(&self, entry: &DirEntry) -> bool;
  // Process the entry. It will be an entry for a regular file.
  fn process_file_entry(&mut self, entry: &DirEntry) -> Result<()>;

  // After process_dirs has finished crawling, call this.
  fn done_processing(&mut self) -> Result<()> {
    Ok(())
  }

  // Handle an error while processing entries. Default behavior just calls println!.
  // Implementors can override this freely.
  fn handle_error(&self, error: &Error) {
    println!("{:?}", error);
  }

  // More general filter function. Default behavior includes all non-files, and
  // calls should_process_file() for regular files.
  // Implementors are free to override this, but should note that should_process_file
  // will not be called unless they call it.
  fn should_process(&self, entry: &DirEntry) -> bool {
    if entry.file_type().is_file() {
      return self.should_process_file(entry);
    }
    true
  }

  // Crawl the supplied dirs.
  // Implementors are free to override this, but should be aware that they become responsible
  // for all aspects of the crawl.
  fn process_dirs<T>(&mut self, dirs: Vec<T>) -> Result<()>
    where T: AsRef<Path> {
    for dir in dirs {
      for entry in WalkDir::new(dir) {
        match entry {
          Err(error) => self.handle_error(&Error::from(error)),
          Ok(entry) => {
            if self.should_process(&entry) {
              match self.process_entry(&entry) {
                Err(err) => self.handle_error(&err),
                _ => {}
              }
            }
          }
        }
      }
    }
    self.done_processing()
  }

  // Process a direntry.
  // Default behavior calls process_file_entry() for regular files and ignores all other entries.
  // Implementors may override, but should be aware that the type-specific processors
  // will not be called.
  fn process_entry(&mut self, entry: &DirEntry) -> Result<()> {
    if entry.file_type().is_file() {
      return self.process_file_entry(entry);
    }
    Ok(())
  }
}
