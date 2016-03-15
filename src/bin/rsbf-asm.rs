extern crate rsbf;

use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::iter::FromIterator;

fn main() {
    let args = Vec::from_iter(std::env::args());
    if args.len() == 2 {
        let path = Path::new(&args[1]);

        let asm = rsbf::asm::compile_file(path).unwrap();
        rsbf::internals::trace(format!("{}", asm));
        let mut asm_file = File::create(format!("{}.S", path.to_str().unwrap())).unwrap();
        asm_file.write_all(&asm.as_bytes()).unwrap();
    } else {
        panic!("Usage: rsbf-asm <file>");
    }
}
