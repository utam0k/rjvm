use std::fs::File;
use std::io::Cursor;
use std::io::Read;

use rjvm::class::Class;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "./samples/HelloWorld.class";
    let mut file = File::open(file_path)?;

    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    let rdr = Cursor::new(data);

    let (class, _rdr) = Class::new(rdr)?;

    println!("{}", class);

    Ok(())
}
