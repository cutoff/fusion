//! I/O utilities for file and directory operations.
//!
//! This module provides utility functions for common I/O operations,
//! such as creating directory structures for files.

use std::fs::create_dir_all;
use std::io;
use std::path::PathBuf;

/// Creates all parent directories for a given path.
///
/// This function is useful when you need to ensure that all directories
/// in a file path exist before creating or writing to the file. It only
/// creates the parent directories, not the file itself.
///
/// # Parameters
///
/// * `path` - The path for which to create parent directories.
///
/// # Returns
///
/// * `io::Result<()>` - Ok if the directories were created successfully or already existed,
///   or an error if the directory creation failed.
///
/// # Examples
///
/// ```
/// use cutoff_common::io::create_dir_all_for;
/// use std::path::PathBuf;
/// use std::fs;
///
/// // Create a temporary path
/// let temp_dir = std::env::temp_dir().join("cutoff_example");
/// let file_path = temp_dir.join("nested/dirs/file.txt");
///
/// // Clean up any existing directory
/// let _ = fs::remove_dir_all(&temp_dir);
///
/// // Create parent directories
/// create_dir_all_for(file_path.clone()).unwrap();
///
/// // Verify that the parent directories were created
/// assert!(file_path.parent().unwrap().exists());
///
/// // Clean up
/// let _ = fs::remove_dir_all(&temp_dir);
/// ```
///
/// # Note
///
/// This function does not create the file itself, only its parent directories.
/// If the path has no parent (e.g., it's a root directory), this function does nothing.
pub fn create_dir_all_for(path: PathBuf) -> io::Result<()> {
    // If the path has a parent, create all directories in the parent path
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_create_dir_all_for() {
        // Create a temporary directory for testing
        let temp_dir = std::env::temp_dir().join("cutoff_common_test");
        let test_path = temp_dir.join("dir1/dir2/dir3/file.txt");

        // Clean up any existing test directory
        let _ = fs::remove_dir_all(&temp_dir);

        // Test creating directories for a file
        assert!(create_dir_all_for(test_path.clone()).is_ok());

        // Verify that the parent directories were created
        assert!(Path::new(&temp_dir.join("dir1/dir2/dir3")).exists());

        // Verify that the file itself was not created
        assert!(!test_path.exists());

        // Test with a path that has no parent (root directory)
        let root_path = PathBuf::from("/");
        assert!(create_dir_all_for(root_path).is_ok());

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_create_dir_all_for_existing_dir() {
        // Create a temporary directory for testing
        let temp_dir = std::env::temp_dir().join("cutoff_common_test_existing");
        let test_path = temp_dir.join("existing/file.txt");

        // Clean up any existing test directory
        let _ = fs::remove_dir_all(&temp_dir);

        // Create the directory structure first
        fs::create_dir_all(temp_dir.join("existing")).unwrap();

        // Test creating directories for a file in an existing directory
        assert!(create_dir_all_for(test_path).is_ok());

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
