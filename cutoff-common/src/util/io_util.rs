use std::fs::create_dir_all;
use std::io;
use std::path::PathBuf;

pub fn create_dir_all_for(path: PathBuf) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    Ok(())
}
