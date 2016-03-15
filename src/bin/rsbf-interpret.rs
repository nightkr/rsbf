extern crate rsbf;

use std::path::Path;
use std::io::{stdin, stdout};
use std::iter::FromIterator;

fn main() {
    let args = Vec::from_iter(std::env::args());
    if args.len() == 2 {
        let path = Path::new(&args[1]);

        rsbf::interp::run_file(&mut stdin(), &mut stdout(), path).unwrap();
    } else {
        panic!("Usage: rsbf-interpret <file>");
    }
}
