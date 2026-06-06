use std::io;
use crate::file::FileHandler;

mod file;

fn main() -> io::Result<()> {
    let file_path = "text-files/example.txt";

    // 1. Initialize the class/struct
    let mut handler = FileHandler::new(file_path)?;
    println!("File opened successfully.");

    // 2. Write data to the file
    handler.write_all("Hello, Rust world!\n")?;
    println!("Initial text written.");

    // 3. Append data to the file
    handler.append("Appending a new line.\n")?;
    println!("New line appended.");

    // 4. Read the file contents back
    let contents = handler.read_to_string()?;
    println!("\n--- File Contents ---");
    print!("{}", contents);
    println!("---------------------");

    Ok(())
}



