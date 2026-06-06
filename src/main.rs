use std::io;
use crate::file::FileHandler;

mod file;

fn main() -> io::Result<()> {
    file_test().expect("File test failed!");
    Ok(())
}

fn file_test() -> io::Result<()> {
    let file_path = "text-files/example.txt";
    let mut handler = FileHandler::new(file_path)?;

    handler.clear()?;
    handler.write("Bullet Points:\n\n")?;
    handler.append("- Message one\n")?;
    handler.append("- Message two\n")?;
    handler.append("- Message three\n")?;

    let contents = handler.read()?;
    print!("\n{}", contents);

    Ok(())
}



