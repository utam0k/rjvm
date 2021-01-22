use std::fs::File;
use std::io::Cursor;
use std::io::Read;

use rjvm::class::Class;
use rjvm::vm::VM;

use clap::Clap;

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "uttam0k <k0ma@utam0k.jp>")]
struct Opts {
    class_file: String,
    #[clap(short)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();
    let mut file = File::open(opts.class_file)?;

    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    let rdr = Cursor::new(data);

    let (class, _rdr) = Class::new(rdr)?;

    if opts.verbose {
        println!("{:?}", class);
    } else {
        let mut vm = VM::new(class);
        vm.exec()?;
    }

    Ok(())
}
