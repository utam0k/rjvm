use std::env;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;

use rjvm::class::Class;
use rjvm::vm::VM;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("please give the path to a class file.");
    let mut file = File::open(file_path)?;

    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    let rdr = Cursor::new(data);

    let (class, _rdr) = Class::new(rdr)?;

    println!("---------- Class information ----------");
    println!("{:?}", class);

    let mut vm = VM::new(class);
    println!("---------- Execution output ----------");
    vm.exec()?;

    Ok(())
}
