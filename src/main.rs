use std::fmt;
use std::fs::File;
use std::io::Read;
use std::mem::{size_of, transmute};

#[repr(C)]
struct Class {
    magic: [u8; 4],
    minor_version: [u8; 2],
    major_version: [u8; 2],
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "magic: ")?;
        for b in self.magic.iter() {
            write!(f, "{:x}", b)?;
        }
        writeln!(f)?;
        write!(f, "minor_version: ")?;
        for b in self.minor_version.iter() {
            write!(f, "{}", b)?;
        }
        writeln!(f)?;
        write!(f, "major_version: ")?;
        for b in self.major_version.iter() {
            write!(f, "{}", b)?;
        }
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
