// rhs -> number
#[derive(Debug, PartialEq)]
enum Rhs {
    Number(i32),
}

// lhs -> identifier
//      | '*' identifier
#[derive(Debug, PartialEq)]
enum Lhs {
    Pointer(String),
    Dereference(String),
}

// expression -> lhs '+=' rhs
//             | lhs '-=' rhs
//             | identifier '(' ')'
//             | lhs
#[derive(Debug, PartialEq)]
enum Expression {
    AssignAdd(Lhs, Rhs),
    AssignSub(Lhs, Rhs),
    FunctionCall(String),
    Lhs(Lhs),
}

// statement  -> expression ';'
//             | block
//             | 'while' expression block
#[derive(Debug, PartialEq)]
enum Statement {
    Expression(Expression),
    Block(Box<Block>),
    While(Expression, Box<Block>),
}

// statements -> statement statement
//             | statement
#[derive(Debug, PartialEq)]
enum Statements {
    Statements(Statement, Statement),
    Statement(Statement),
}

// block -> '{' statements '}'
#[derive(Debug, PartialEq)]
enum Block {
    Statements(Statements),
}

// program -> statements
#[derive(Debug, PartialEq)]
enum Program {
    Statements(Statements),
}
