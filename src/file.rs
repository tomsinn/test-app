use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::path::Path;

/// A struct that encapsulates file operations for reading and writing.
pub struct FileHandler {
    file: File,
}

impl FileHandler {
    /// Opens an existing file or creates a new one with read and write permissions.
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true) // Creates the file if it does not exist
            .open(path)?;

        Ok(FileHandler { file })
    }

    /// Reads the entire contents of the file into a String.
    pub fn read_to_string(&mut self) -> io::Result<String> {
        // Reset the file pointer to the beginning before reading
        self.file.seek(SeekFrom::Start(0))?;

        let mut contents = String::new();
        self.file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    /// Overwrites the file with the provided text and truncates old content.
    pub fn write_all(&mut self, text: &str) -> io::Result<()> {
        // Reset file pointer and truncate the file length to 0
        self.file.seek(SeekFrom::Start(0))?;
        self.file.set_len(0)?;

        self.file.write_all(text.as_bytes())?;
        // Ensure data is completely flushed to the disk
        self.file.flush()?;
        Ok(())
    }

    /// Appends text to the end of the file.
    pub fn append(&mut self, text: &str) -> io::Result<()> {
        // Move the file pointer to the end of the file
        self.file.seek(SeekFrom::End(0))?;

        self.file.write_all(text.as_bytes())?;
        self.file.flush()?;
        Ok(())
    }
}
