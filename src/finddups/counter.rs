use std::io;
use std::io::Write;
use term_size;

pub struct Counter {
  count: isize,
  width: usize,
  pad: String,
}

impl Counter {
  pub fn new() -> Counter {
    let screen_width = term_size::dimensions().map_or(80, |(w, _)| w);
    let mut pad = String::with_capacity(screen_width);
    for _ in 0..screen_width {
      pad.push(' ');
    }
    Counter {
      count: 0,
      width: screen_width,
      pad: pad,
    }
  }

  pub fn inc(&mut self, suffix: Option<&str>) -> io::Result<()> {
    self.count += 1;
    if self.count % 10 == 0 {
      try!(self.output(suffix));
    }
    Ok(())
  }

  pub fn done(&mut self, suffix: Option<&str>) -> io::Result<()> {
    try!(self.output(suffix));
    println!("");
    Ok(())
  }

  fn output(&self, suffix: Option<&str>) -> io::Result<()> {
    let count_string = self.count.to_string();
    print!("\r{}", count_string);
    if let Some(suffix) = suffix {
      // Subtract extra 2 for the ": ".
      let spaces_left = self.width - count_string.len() - 2;
      let suffix_len = suffix.len();
      if suffix_len > spaces_left {
        let suffix = &suffix[(suffix_len - spaces_left)..];
        print!(": {}", suffix);
      } else {
        print!(": {}", suffix);
        print!("{}", &self.pad[0..(spaces_left - suffix_len)])
      }
    }
    io::stdout().flush()
  }
}
