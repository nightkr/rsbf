use bf::*;
use std::path::Path;
use std::fs::File;
use std::io::{Read, Result};

#[derive(Copy, Clone)]
pub struct TokenReader<'a> {
    src: &'a [u8],
    pub program_counter: usize
}

impl <'a> TokenReader<'a> {
    pub fn new(src: &'a [u8]) -> TokenReader<'a> {
        TokenReader {
            src: src,
            program_counter: 0
        }
    }

    fn read_next(&mut self) -> Option<char> {
        let chr = self.src.get(self.program_counter);
        chr.map( |x| {
            self.program_counter += 1;
            *x as char
        })
    }

    pub fn find_closing_brace(&mut self) -> Option<usize> {
        let pc = self.program_counter;
        let mut level = 1;
        loop {
            match self.next() {
                Some(BFToken::JumpFwd) => level += 1,
                Some(BFToken::JumpBwd) => {
                    level -= 1;
                    if level == 0 {
                        let pos = self.program_counter;
                        self.program_counter = pc;
                        return Some(pos - 1)
                    }
                },
                Some(_) => {},
                None => {
                    self.program_counter = pc;
                    return None
                }
            }
        }
    }
}

impl <'a> Iterator for TokenReader<'a> {
    type Item = BFToken;

    fn next(&mut self) -> Option<BFToken> {
        loop {
            let token = self.read_next().map(BFToken::parse);
            match token {
                Some(Some(tok)) => return Some(tok),
                Some(None) => {},
                None => return None
            }
        }
    }
}

pub fn read_file(path: &Path) -> Result<Vec<u8>> {
    let mut file = try!(File::open(path));
    let mut buf = Vec::<u8>::new();
    try!(file.read_to_end(&mut buf));
    Ok(buf)
}

pub fn trace(s: String) {
    if cfg!(feature = "trace") {
        println!("{}", s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_unmatched_bracket() {
        let mut tr = TokenReader::new("[[]".as_bytes());
        tr.next();
        assert_eq!(tr.find_closing_brace(), None);
    }

    #[test]
    fn find_nested_bracket() {
        let mut tr = TokenReader::new("[[]]".as_bytes());
        tr.next();
        assert_eq!(tr.find_closing_brace(), Some(3));
        tr.next();
        assert_eq!(tr.find_closing_brace(), Some(2));
    }
}
