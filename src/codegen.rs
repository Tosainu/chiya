pub mod emitter;
pub mod llvm;

use crate::parser::{Block, Expression, Lhs, Program, Rhs, Statement, Statements};

#[derive(Debug, PartialEq, failure::Fail)]
pub enum CodegenError {
    #[fail(display = "invalid variable name: '{}'", name)]
    InvalidVariableName { name: String },

    #[fail(display = "invalid function name: '{}'", name)]
    InvalidFunctionName { name: String },

    #[fail(display = "not implemented")]
    NotImplemented,
}

pub fn gen<E: emitter::Emitter>(emitter: &mut E, tree: &Program) -> Result<String, CodegenError> {
    match tree {
        Program::Statements(ss) => {
            let header = emitter.emit_header();
            let body = statements(emitter, ss)?;
            let footer = emitter.emit_footer();
            Ok(format!("{}{}{}", header, body, footer))
        }
    }
}

fn statements<E: emitter::Emitter>(
    emitter: &mut E,
    tree: &Statements,
) -> Result<String, CodegenError> {
    match tree {
        Statements::Statement(s) => statement(emitter, s),
        Statements::Statements(ss, s) => statements(emitter, ss)
            .and_then(|s0| statement(emitter, s).map(|s1| format!("{}{}", s0, s1))),
    }
}

fn statement<E: emitter::Emitter>(
    emitter: &mut E,
    tree: &Statement,
) -> Result<String, CodegenError> {
    match tree {
        Statement::Expression(e) => expression(emitter, e),
        Statement::Block(b) => block(emitter, b),
        Statement::While(e, b) => while_s(emitter, e, b),
    }
}

fn block<E: emitter::Emitter>(emitter: &mut E, tree: &Block) -> Result<String, CodegenError> {
    match tree {
        Block::Statements(ss) => statements(emitter, ss),
    }
}

fn expression<E: emitter::Emitter>(
    emitter: &mut E,
    tree: &Expression,
) -> Result<String, CodegenError> {
    match tree {
        Expression::AssignAdd(Lhs::Pointer(p), Rhs::Number(n)) => move_ptr(emitter, p, *n),
        Expression::AssignSub(Lhs::Pointer(p), Rhs::Number(n)) => move_ptr(emitter, p, -*n),
        Expression::AssignAdd(Lhs::Dereference(p), Rhs::Number(n)) => add(emitter, p, *n),
        Expression::AssignSub(Lhs::Dereference(p), Rhs::Number(n)) => add(emitter, p, -*n),
        Expression::FunctionCall(Lhs::Pointer(p)) => function_call(emitter, p),
        _ => Err(CodegenError::NotImplemented),
    }
}

fn while_s<E: emitter::Emitter>(
    emitter: &mut E,
    cond: &Expression,
    body: &Block,
) -> Result<String, CodegenError> {
    match cond {
        Expression::Lhs(Lhs::Dereference(ptr)) => {
            if ptr != "ptr" {
                Err(CodegenError::InvalidVariableName {
                    name: ptr.to_string(),
                })
            } else {
                let header = emitter.emit_loop_begin();
                let body = block(emitter, body)?;
                let footer = emitter.emit_loop_end();
                Ok(format!("{}{}{}", header, body, footer))
            }
        }

        _ => Err(CodegenError::NotImplemented),
    }
}

fn move_ptr<E: emitter::Emitter>(
    emitter: &mut E,
    ptr: &str,
    offset: i32,
) -> Result<String, CodegenError> {
    if ptr != "ptr" {
        Err(CodegenError::InvalidVariableName {
            name: ptr.to_string(),
        })
    } else {
        Ok(emitter.emit_move_ptr(offset))
    }
}

fn add<E: emitter::Emitter>(emitter: &mut E, ptr: &str, n: i32) -> Result<String, CodegenError> {
    if ptr != "ptr" {
        Err(CodegenError::InvalidVariableName {
            name: ptr.to_string(),
        })
    } else {
        Ok(emitter.emit_add(n))
    }
}

fn function_call<E: emitter::Emitter>(
    emitter: &mut E,
    funcname: &str,
) -> Result<String, CodegenError> {
    match funcname {
        "getchar" => Ok(emitter.emit_call_getchar()),
        "putchar" => Ok(emitter.emit_call_putchar()),
        _ => Err(CodegenError::InvalidFunctionName {
            name: funcname.to_string(),
        }),
    }
}
