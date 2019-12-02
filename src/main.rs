use std::fs::File;
use std::io::Cursor;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "./samples/HelloWorld.class";
    let mut f = File::open(&file_path)?;
    let mut buf = Vec::new();
    let magic: &mut [u8] = &mut [0; 4];

    let _ = f.read_to_end(&mut buf)?;
    let mut cur = Cursor::new(buf);
    cur.read_exact(magic)?;
    for b in magic {
        print!("{:x}", b);
    }
    println!();
    Ok(())
}
