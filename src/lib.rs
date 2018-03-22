#![deny(warnings, missing_docs)]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]

//! mkdir -p

use std::fs::create_dir_all;
use std::path::Path;
use std::io;

/// Create a new directory, ignore if it already exists.
pub fn mkdirp<P: AsRef<Path>>(path: &P) -> io::Result<()> {
  if let Err(e) = create_dir_all(path) {
    if e.kind() != io::ErrorKind::AlreadyExists {
      return Err(e);
    }
  }
  Ok(())
}
