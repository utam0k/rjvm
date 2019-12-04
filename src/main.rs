use std::fmt;
use std::fs::File;
use std::io::Read;
use std::mem::{size_of, transmute};

#[repr(C)]
struct Class {
    magic: u32,
    minor_version: u16,
    major_version: u16,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "magic: {:x}", self.magic)?;
        writeln!(f, "minor_version: {}", self.minor_version)?;
        writeln!(f, "major_version: {}", self.major_version)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "./samples/HelloWorld.class";
    let mut file = File::open(file_path)?;

    let class: Class = {
        let mut c = [0u8; size_of::<Class>()];
        file.read_exact(&mut c[..])?;
        unsafe { transmute(c) }
    };

    println!("{}", class);

    Ok(())
}
