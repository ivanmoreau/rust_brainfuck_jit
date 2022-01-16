use std::fs;

use dynasmrt::{DynasmApi, dynasm, DynasmLabelApi};
mod parser;

pub extern "sysv64" fn print_s(ax: u64) {
    let a = ax as u8;
    let a = a as char;
    print!("{}", a);
}

pub extern "sysv64" fn read_s() -> u64 {
    let input: Option<u64> = std::io::Read::bytes(std::io::stdin()) 
                    .next()
                    .and_then(|result| result.ok())
                    .map(|byte| byte as u64);
    input.unwrap()
}

pub extern "sysv64" fn print_buffer(ptr: *const u64) {
    let aa: &[u64; 10] = unsafe { std::mem::transmute( ptr ) };
    //println!("Buffer: {:?}", aa);
}

fn ex_rec(mut ops: dynasmrt::Assembler<dynasmrt::x64::X64Relocation>,
    mut din_lab: Vec<(dynasmrt::DynamicLabel, dynasmrt::DynamicLabel)>,
    p: Vec<parser::Instruction>) -> 
    (dynasmrt::Assembler<dynasmrt::x64::X64Relocation>,
        Vec<(dynasmrt::DynamicLabel, dynasmrt::DynamicLabel)>) {
    for e in p {
        match e {
            parser::Instruction::Inc(x) => dynasm!(ops
                ; .arch x64
                ; call ->ppr
                ; mov rax, QWORD x as _
                ; add rdx, rax
                ),
            parser::Instruction::Dec(x) => dynasm!(ops
                ; .arch x64
                ; call ->ppr
                ; mov rax, QWORD x as _
                ; sub rdx, rax
                ),
            parser::Instruction::Add(x) => dynasm!(ops
                ; .arch x64
                ; call ->ppr
                ; mov rax, QWORD x as _
                ; add [rdi + rdx * 8], rax
                ),
            parser::Instruction::Min(x) => dynasm!(ops
                ; .arch x64
                ; call ->ppr
                ; mov rax, QWORD x as _
                ; sub [rdi + rdx * 8], rax
                ),
            parser::Instruction::Out => {
                dynasm!(ops
                    ; .arch x64
                    ; push rdi
                    ; push rdx
                    ; mov rdi, [rdi + rdx * 8]
                    ; mov rax, QWORD print_s as _
                    ; call rax
                    ; pop rdx
                    ; pop rdi
                );

            },
            parser::Instruction::Int => {
                dynasm!(ops
                    ; .arch x64
                    ; push rdi
                    ; push rdx
                    ; mov rax, QWORD read_s as _
                    ; call rax
                    ; pop rdx
                    ; pop rdi
                    ; mov QWORD [rdi + rdx * 8], rax
                );
            },
            parser::Instruction::While(x) => {
                let ini_l = ops.new_dynamic_label();
                let end_l = ops.new_dynamic_label();
                din_lab.push((ini_l, end_l));
                dynasm!(ops
                    ; .arch x64
                    ; call ->ppr
                    ; mov rcx, [rdi + rdx * 8]
                    ; cmp rcx, 0
                    ; jz =>end_l
                    ;=>ini_l
                );
                let xx = ex_rec(ops, din_lab, x);
                ops = xx.0; din_lab = xx.1;
                let (ini_l, end_l) = din_lab.pop().unwrap();
                dynasm!(ops
                    ; .arch x64
                    ; mov rax, [rdi + rdx * 8]
                    ; cmp rax, 0
                    ; jnz =>ini_l
                    ;=>end_l
                );
            },
        }
    }
    (ops, din_lab)
}

extern "C" {fn malloc(size: u64) -> *mut std::ffi::c_void;}

fn execute(program: String) {
    let p: Vec<parser::Instruction> = parser::parse(program);
    let mut buffer: [u64; 10] = [0u64;10];
    let mut ops: dynasmrt::Assembler<dynasmrt::x64::X64Relocation> 
        = dynasmrt::x64::Assembler::new().unwrap();

    let mut din_lab: Vec<(dynasmrt::DynamicLabel, dynasmrt::DynamicLabel)> 
        = vec![];
    
    let ptr = &buffer as *const u64;
    //Debug
    //println!("ptr 1: {:?}", ptr);
    //print_buffer(ptr);
    dynasm!(ops
        ; .arch x64
        ; ->ppr:
        ; push rdi
        ; push rdx
        ; push rax
        ; mov rax, QWORD print_buffer as _
        ; call rax
        ; pop rax
        ; pop rdx
        ; pop rdi
        ; ret
    );
    let o = ops.offset();
    dynasm!(ops
        ; .arch x64
        ; push rbp
        ; push rdi
        ; push rsi
        ; xor rcx, rcx
        ; mov rax, QWORD malloc as _
        ; mov rdi, 1000
        ; call rax
        ; mov rax, QWORD ptr as _
        ; mov rdi, rax
        ; push rdi
        ; mov rax, QWORD print_s as _
        ; call rax
        ; pop rdi
        ; xor rdx, rdx
        ; call ->ppr
        //; push rdi
        //; push rdx
        //; mov rax, QWORD read_s as _
        //; call rax
        //; pop rdx
        //; pop rdi
    ); //        ; add [rcx], 4   -- ; mov rsi, QWORD ptr2 as _
    let (mut ops, _) = ex_rec(ops, din_lab, p);
    dynasm!(ops
        ; .arch x64
        ; pop rsi
        ; pop rdi
        ; pop rbp
        ; ret
    );
    let buf = ops.finalize().unwrap();
    let hello_fn: extern "win64" fn() -> bool = unsafe { std::mem::transmute(buf.ptr(o)) };
    //Debug
    let ñ = buf.as_ref();   // Doesn't work without this wtf?? (It's probably an optimization of rustc).
    extract_bin(ñ);   // Doesn't work without this wtf?? (It's probably an optimization of rustc).
    hello_fn();
    //println!("{:?}", &buffer);
}

fn extract_bin(a: &[u8]) -> std::io::Result<()> {
    
    let mut file = fs::File::create("testbin")?;
    std::io::Write::write_all(&mut file, a)?;

    Ok(())
}

fn main() {
    let content = fs::read_to_string("/Users/ivanmolinarebolledo/rust_brainfuck_jit/file.bs");
    //println!("ss");
    execute(content.unwrap());
    //let res = parser::parse(content.unwrap());
    //println!("Hello, world! {:?}", res);
    //println!("ss");
}
