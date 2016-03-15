use bf::*;
use bf::internals::*;
use std::path::Path;
use std::io::{Result, Read, Write};

struct BFState<'a> {
    stdin: &'a mut Read,
    stdout: &'a mut Write,
    src: TokenReader<'a>,
    stack: Vec<usize>,
    cells: [u8; 30000],
    cell_ptr: usize
}

impl <'a> BFState<'a> {
    fn new(stdin: &'a mut Read, stdout: &'a mut Write, src: &'a [u8]) -> BFState<'a> {
        BFState {
            stdin: stdin,
            stdout: stdout,
            src: TokenReader::new(src),
            stack: Vec::new(),
            cells: [0; 30000],
            cell_ptr: 0
        }
    }

    fn curr_cell(&self) -> u8 {
        self.cells[self.cell_ptr]
    }

    fn curr_cell_mut(&mut self) -> &mut u8 {
        &mut self.cells[self.cell_ptr]
    }

    fn run_token(&mut self, token: BFToken) -> Result<()> {
        match token {
            BFToken::PtrIncr => {
                self.cell_ptr = (self.cell_ptr + 1) % self.cells.len();
                Ok(())
            },
            BFToken::PtrDecr => {
                self.cell_ptr = match self.cell_ptr {
                    0 => self.cells.len() - 1,
                    x => x - 1
                };
                Ok(())
            },
            BFToken::CellIncr => {
                *self.curr_cell_mut() = self.curr_cell().wrapping_add(1);
                Ok(())
            },
            BFToken::CellDecr => {
                *self.curr_cell_mut() = self.curr_cell().wrapping_sub(1);
                Ok(())
            },
            BFToken::CellOut => {
                let cell = *self.curr_cell_mut();
                write!(self.stdout, "{}", cell as u8 as char)
            },
            BFToken::CellIn => {
                let mut buf: [u8; 1] = [0];
                try!(self.stdin.read(&mut buf));
                *self.curr_cell_mut() = buf.get(0).map(|x| *x).unwrap_or(0);
                Ok(())
            },
            BFToken::JumpFwd => {
                self.stack.push(self.src.program_counter - 1);
                if self.curr_cell() == 0 {
                    self.src.program_counter = self.src.find_closing_brace().unwrap();
                }
                Ok(())
            },
            BFToken::JumpBwd => {
                let target = self.stack.pop().unwrap();
                if self.curr_cell() != 0 {
                    self.src.program_counter = target;
                }
                Ok(())
            },
            BFToken::Break => panic!("Breakpoint hit!")
        }
    }

    fn run_next(&mut self) -> Result<bool> {
        match self.src.next() {
            Some(tok) => {
                let pc = self.src.program_counter;
                try!(self.run_token(tok));
                internals::trace(format!("{:?} {} {} {}", tok, pc - 1, self.cell_ptr, self.curr_cell()));
                Ok(true)
            },
            None => Ok(false)
        }
    }
}

pub fn run_buf(stdin: &mut Read, stdout: &mut Write, buf: &[u8]) -> Result<()> {
    let mut state = BFState::new(stdin, stdout, buf);
    while try!(state.run_next()) {
    }
    Ok(())
}

pub fn run_file(stdin: &mut Read, stdout: &mut Write, path: &Path) -> Result<()> {
    let buf = try!(read_file(path));
    run_buf(stdin, stdout, &buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::str;

    fn expect_output(path: &str, input: &str, output: &str) {
        use std::io;
        let mut stdin = input.as_bytes();
        let mut stdout = Vec::new();
        run_file(&mut stdin, &mut stdout, Path::new(path)).unwrap();
        assert_eq!(str::from_utf8(&stdout).unwrap(), output);
    }

    #[test]
    fn hello_world() {
        expect_output("tests/hello.bf", "", "Hello World!\n");
        expect_output("tests/hello-tricky.bf", "", "Hello World!\n");
        expect_output("tests/hello-primo.bf", "", "Hello, World!");
    }

    #[test]
    fn cat() {
        expect_output("tests/cat.bf", "asdf", "asdf");
        expect_output("tests/cat.bf", "blah", "blah");
    }
}
