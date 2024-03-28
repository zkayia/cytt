
use std::{path::Path, io, fs};


// https://nick.groenen.me/notes/recursively-copy-files-in-rust
pub fn copy_recursively(source: impl AsRef<Path>, destination: impl AsRef<Path>) -> io::Result<()> {
  
  fs::create_dir_all(&destination)?;
  
  for entry in fs::read_dir(source)? {
    
    let entry = entry?;
    let filetype = entry.file_type()?;
    
    if filetype.is_dir() {
      copy_recursively(entry.path(), destination.as_ref().join(entry.file_name()))?;
    } else {
      fs::copy(entry.path(), destination.as_ref().join(entry.file_name()))?;
    }
  }
  Ok(())
}