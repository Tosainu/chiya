use std::io::Read;

use chiya::codegen::{self, emitter::Emitter, llvm::LLVM};
use chiya::parser;
use chiya::token;

fn main() -> Result<(), failure::Error> {
    let args = std::env::args().collect::<Vec<_>>();

    let debug = args.iter().any(|a| *a == "--debug");
    let bf = args.iter().any(|a| *a == "--bf");

    let mut src = String::new();
    std::io::stdin().read_to_string(&mut src)?;

    let e = LLVM::new();

    if bf {
        compile_bf(e, &src)?;
    } else {
        compile(e, &src, debug)?;
    }

    Ok(())
}

fn compile<E: Emitter>(mut e: E, src: &str, debug: bool) -> Result<(), failure::Error> {
    let tokens = token::tokenize(&src)?;
    if debug {
        eprintln!("tokens: {:?}", tokens);
    }

    let tree = parser::program(&tokens).ok_or_else(|| failure::format_err!("parse error"))?;
    if debug {
        eprintln!("syntax tree:\n{:#?}", tree.1);
    }

    let code = codegen::gen(&mut e, &tree.1)?;
    println!("{}", code);

    Ok(())
}

fn compile_bf<E: Emitter>(mut e: E, src: &str) -> Result<(), failure::Error> {
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
