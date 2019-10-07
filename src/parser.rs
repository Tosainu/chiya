// rhs -> number
#[derive(Debug, PartialEq)]
pub enum Rhs {
    Number(i32),
}

// lhs -> identifier
//      | '*' identifier
#[derive(Debug, PartialEq)]
pub enum Lhs {
    Pointer(String),
    Dereference(String),
}

// expression -> lhs '+=' rhs
//             | lhs '-=' rhs
//             | identifier '(' ')'
//             | lhs
#[derive(Debug, PartialEq)]
pub enum Expression {
    AssignAdd(Lhs, Rhs),
    AssignSub(Lhs, Rhs),
    FunctionCall(String),
    Lhs(Lhs),
}

// statement  -> expression ';'
//             | block
//             | 'while' expression block
#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression(Expression),
    Block(Box<Block>),
    While(Expression, Box<Block>),
}

// statements -> statement statement
//             | statement
#[derive(Debug, PartialEq)]
pub enum Statements {
    Statements(Statement, Statement),
    Statement(Statement),
}

// block -> '{' statements '}'
#[derive(Debug, PartialEq)]
pub enum Block {
    Statements(Statements),
}

// program -> statements
#[derive(Debug, PartialEq)]
pub enum Program {
    Statements(Statements),
}
