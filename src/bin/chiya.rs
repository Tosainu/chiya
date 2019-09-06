use std::io::Read;

use chiya::codegen::emitter::Emitter;
use chiya::codegen::llvm::LLVM;

fn main() -> std::io::Result<()> {
    let mut src = String::new();
    std::io::stdin().read_to_string(&mut src)?;

    let mut e = LLVM::new();

    println!("{}", e.emit_header());
    for c in src.chars() {
        let l = match c {
            '>' => e.emit_move_ptr(1),
            '<' => e.emit_move_ptr(-1),
            '+' => e.emit_add(1),
            '-' => e.emit_add(-1),
            '.' => e.emit_call_putchar(),
            ',' => e.emit_call_getchar(),
            '[' => e.emit_loop_begin(),
            ']' => e.emit_loop_end(),
            _ => continue,
        };
        println!("{}", l);
    }
    println!("{}", e.emit_footer());

    Ok(())
}
