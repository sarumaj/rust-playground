use std::error::Error;
use std::fs::OpenOptions;
use std::io::{stdin, Read, Seek, SeekFrom, Write};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Enter the path to the file:");
    let mut path = String::new();
    stdin().read_line(&mut path)?;
    let path = path.trim();

    let mut file = OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open(path.trim())?;
    println!("File opened successfully: {:?}", file);

    println!("Do you want to delete the file? (y/n)");
    let mut delete = String::new();
    stdin().read_line(&mut delete)?;
    if delete.trim() == "y" {
        std::fs::remove_file(&path)?;
        println!("File deleted successfully.");
        return Ok(());
    }

    println!("Enter the text to write to the file:");
    let mut text = String::new();
    stdin().read_line(&mut text)?;
    file.write_all(text.as_bytes())?;
    println!("Text written successfully.");

    file.seek(SeekFrom::Start(0))?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let content = content.trim_end();
    println!("File contents:\n------------\n{}\n-------------", content);

    Ok(())
}
