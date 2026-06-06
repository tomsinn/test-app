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
            .create(true)
            .open(path)?;

        Ok(FileHandler { file })
    }

    pub fn read(&mut self) -> io::Result<String> {
        self.file.seek(SeekFrom::Start(0))?;
        let mut contents = String::new();
        self.file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn clear(&mut self) -> io::Result<()> {
        self.file.seek(SeekFrom::Start(0))?;
        self.file.set_len(0)?;
        self.file.write_all("".as_bytes())?;
        self.file.flush()?;
        Ok(())
    }

    pub fn write(&mut self, text: &str) -> io::Result<()> {
        self.file.seek(SeekFrom::Start(0))?;
        self.file.set_len(0)?;
        self.file.write_all(text.as_bytes())?;
        self.file.flush()?;
        Ok(())
    }

    pub fn append(&mut self, text: &str) -> io::Result<()> {
        self.file.seek(SeekFrom::End(0))?;
        self.file.write_all(text.as_bytes())?;
        self.file.flush()?;
        Ok(())
    }
}
