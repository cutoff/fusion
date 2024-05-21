use std::path::PathBuf;

use glob::glob;

pub fn get_matching_paths_by_glob_pattern(glob_pattern: &str) -> Result<Vec<PathBuf>, glob::PatternError> {
    let mut paths = vec![];

    for entry in glob(glob_pattern)? {
        match entry {
            Ok(path) => paths.push(path),
            Err(e) => println!("{:?}", e),
        }
    }

    Ok(paths)
}
