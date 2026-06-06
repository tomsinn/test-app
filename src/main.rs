use std::io;
use crate::db::query_db;
use crate::file::FileHandler;

mod file;
mod db;

fn main() -> io::Result<()> {
    file_test().expect("File test failed!");
    db_test().expect("Database test failed!");
    Ok(())
}

fn db_test() -> Result<(), sqlx::Error> {
    query_db()
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



