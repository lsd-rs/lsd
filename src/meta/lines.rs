use crate::color::{ColoredString, Colors, Elem};
use std::fs::{File, Metadata};
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

/// Maximum file size to scan for line counting (10MB).
/// Files larger than this will not have their lines counted to avoid performance issues.
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

/// Size of buffer to check for binary files (8KB).
/// We check this many bytes at the start of a file for null bytes to detect binary files.
const BINARY_CHECK_SIZE: usize = 8192;

/// Represents the line count of a file.
///
/// For regular text files, contains Some(count). For directories, binary files,
/// files that are too large, or files that cannot be read, contains None.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Lines {
    count: Option<u64>,
}

impl Lines {
    /// Create a Lines instance from a total count.
    ///
    /// This is used when calculating total lines for directories,
    /// where we sum up the line counts of all contained files.
    pub fn from_total(count: u64) -> Self {
        Self { count: Some(count) }
    }

    /// Create a Lines instance by counting lines in a file.
    ///
    /// Returns None for:
    /// - Directories and non-regular files
    /// - Files larger than MAX_FILE_SIZE
    /// - Binary files (detected by null bytes)
    /// - Files that cannot be read
    pub fn from_path(path: &Path, metadata: &Metadata) -> Self {
        // Only count lines for regular files
        if !metadata.is_file() {
            return Self { count: None };
        }

        // Skip files that are too large to avoid performance issues
        if metadata.len() > MAX_FILE_SIZE {
            return Self { count: None };
        }

        // Attempt to count lines, returning None on any error
        match Self::count_lines(path) {
            Ok(count) => Self { count: Some(count) },
            Err(_) => Self { count: None },
        }
    }

    /// Count the number of lines in a file.
    ///
    /// Returns 0 for binary files (detected by null bytes in first 8KB).
    /// Returns the actual line count for text files.
    fn count_lines(path: &Path) -> std::io::Result<u64> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        // Check if file is binary by scanning for null bytes
        let mut buffer = vec![0; BINARY_CHECK_SIZE];
        let bytes_read = reader.read(&mut buffer)?;

        // Binary files contain null bytes - return 0 to indicate this
        if buffer[..bytes_read].contains(&0) {
            return Ok(0);
        }

        // Reopen file to count lines from the beginning
        drop(reader);
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // Count lines using BufReader's lines iterator
        let mut count = 0u64;
        for _ in reader.lines() {
            count += 1;
        }

        Ok(count)
    }

    /// Render the line count with appropriate coloring.
    ///
    /// Uses file size color scheme:
    /// - Small files (<100 lines): FileSmall color
    /// - Medium files (100-999 lines): FileMedium color
    /// - Large files (>=1000 lines): FileLarge color
    /// - Binary/unreadable files: NoAccess color (displays "-")
    pub fn render(&self, colors: &Colors) -> ColoredString {
        match self.count {
            Some(0) => colors.colorize('-', &Elem::NoAccess),
            Some(c) => {
                let elem = if c >= 1000 {
                    &Elem::FileLarge
                } else if c >= 100 {
                    &Elem::FileMedium
                } else {
                    &Elem::FileSmall
                };
                colors.colorize(c.to_string(), elem)
            }
            None => colors.colorize('-', &Elem::NoAccess),
        }
    }

    /// Get the line count as a string for alignment calculations.
    ///
    /// Returns "-" for binary files, directories, and unreadable files.
    pub fn value_string(&self) -> String {
        match self.count {
            Some(0) => String::from("-"),
            Some(c) => c.to_string(),
            None => String::from("-"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Lines;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_lines_empty_file() {
        let file = NamedTempFile::new().unwrap();
        let meta = file.path().metadata().unwrap();
        let lines = Lines::from_path(file.path(), &meta);
        assert_eq!(lines.count, Some(0));
    }

    #[test]
    fn test_lines_text_file() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "line 1").unwrap();
        writeln!(file, "line 2").unwrap();
        writeln!(file, "line 3").unwrap();
        file.flush().unwrap();

        let meta = file.path().metadata().unwrap();
        let lines = Lines::from_path(file.path(), &meta);
        assert_eq!(lines.count, Some(3));
    }

    #[test]
    fn test_lines_binary_file() {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(&[0u8, 1u8, 2u8, 0u8]).unwrap();
        file.flush().unwrap();

        let meta = file.path().metadata().unwrap();
        let lines = Lines::from_path(file.path(), &meta);
        assert_eq!(lines.count, Some(0)); // Binary files return 0
    }

    #[test]
    fn test_lines_from_total() {
        let lines = Lines::from_total(42);
        assert_eq!(lines.count, Some(42));
    }

    #[test]
    fn test_value_string_text_file() {
        let lines = Lines::from_total(123);
        assert_eq!(lines.value_string(), "123");
    }

    #[test]
    fn test_value_string_binary_file() {
        let lines = Lines { count: Some(0) };
        assert_eq!(lines.value_string(), "-");
    }

    #[test]
    fn test_value_string_none() {
        let lines = Lines { count: None };
        assert_eq!(lines.value_string(), "-");
    }
}
