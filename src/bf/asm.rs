use bf::*;
use bf::internals::*;
use std::path::Path;
use std::io::Result;

struct BFCState<'a> {
    src: TokenReader<'a>,
    target: Vec<String>,
    stack: Vec<usize>
}

impl <'a> BFCState<'a> {
    fn new(src: &[u8]) -> BFCState {
        BFCState {
            src: TokenReader::new(src),
            target: vec![BFCState::asm_header()],
            stack: Vec::new()
        }
    }

    fn finalize(&mut self) {
        self.target.push(format!("bf_stmt_{}:\n", self.src.program_counter - 1));
        self.target.push(BFCState::asm_footer());
    }

    fn asm_header() -> String {
        "
        .intel_syntax
        .data
        .comm cells, 65536
        #cells: .fill 65536, 1, 0
        .text
        .global main

        input:
        push %rax
        push %rbx
        push %rcx
        push %rdx

        call getchar

        pop %rdx
        pop %rcx
        pop %rbx
        pop %rax
        mov [cells+%eax], %rsi

        cmp %esi, -1
        jne input_ret
        movb [cells+%eax], 0

        input_ret:
        ret

        output:
        push %rax
        push %rbx
        push %rcx
        push %rdx

        mov %rdi, [cells+%eax]
        call putchar

        pop %rdx
        pop %rcx
        pop %rbx
        pop %rax
        ret

        main:

        mov %eax, 0
        mov %ecx, 0xFF

".to_string()
    }

    fn asm_footer() -> String {
        "
        exit:
        mov %rbx, 0
        mov %rax, 1
        int 0x80
".to_string()
    }

    fn compile_token(&mut self, token: BFToken) -> String {
        match token {
            BFToken::PtrIncr => "inc %ax\n".to_string(),
            BFToken::PtrDecr => "dec %ax\n".to_string(),
            BFToken::CellIncr => "incb [cells+%eax]\n".to_string(),
            BFToken::CellDecr => "decb [cells+%eax]\n".to_string(),
            BFToken::CellOut => "call output\n".to_string(),
            BFToken::CellIn => "call input\n".to_string(),
            BFToken::JumpFwd => {
                self.stack.push(self.src.program_counter - 1);
                let target = self.src.find_closing_brace().unwrap() + 1;
                format!("test %ecx,[cells+%eax]\njz bf_stmt_{}\n", target).to_string()
            },
            BFToken::JumpBwd => {
                let target = self.stack.pop().unwrap() + 1;
                format!("test %ecx,[cells+%eax]\njnz bf_stmt_{}\n", target).to_string()
            },
            BFToken::Break => "".to_string()
        }
    }

    fn compile_next(&mut self) -> bool {
        let pc = self.src.program_counter;
        match self.src.next() {
            Some(tok) => {
                self.target.push(format!("bf_stmt_{}:\n", pc));
                let asm = self.compile_token(tok);
                self.target.push(asm);
                true
            },
            None => false
        }
    }
}

pub fn compile_buf(buf: &[u8]) -> Result<String> {
    let mut state = BFCState::new(buf);
    while state.compile_next() {
    }
    state.finalize();
    let mut result = String::new();
    for s in state.target {
        result.push_str(&s);
    }
    Ok(result)
}

pub fn compile_file(path: &Path) -> Result<String> {
    let buf = try!(read_file(path));
    compile_buf(&buf)
}
