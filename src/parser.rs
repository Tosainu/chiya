use crate::token::Token;

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

pub fn rhs(tokens: &[Token]) -> Option<(&[Token], Rhs)> {
    tokens.first().and_then(|t| match t {
        Token::Integer(i) => Some((&tokens[1..], Rhs::Number(*i))),
        _ => None,
    })
}

#[test]
fn test_rhs() {
    assert_eq!(rhs(&[]), None);
    assert_eq!(rhs(&[Token::CurlyOpen]), None);
    assert_eq!(rhs(&[Token::Identififier("123".to_owned())]), None);
    assert_eq!(
        rhs(&[Token::Integer(123)]),
        Some((&[] as &[Token], Rhs::Number(123)))
    );
}

pub fn lhs(tokens: &[Token]) -> Option<(&[Token], Lhs)> {
    match (tokens.get(0), tokens.get(1)) {
        (Some(Token::Star), Some(Token::Identififier(s))) => {
            Some((&tokens[2..], Lhs::Dereference(s.to_string())))
        }

        (Some(Token::Identififier(s)), _) => Some((&tokens[1..], Lhs::Pointer(s.to_string()))),

        _ => None,
    }
}

#[test]
fn test_lhs() {
    assert_eq!(lhs(&[]), None);
    assert_eq!(lhs(&[Token::CurlyOpen]), None);
    assert_eq!(lhs(&[Token::Integer(123)]), None);
    assert_eq!(
        lhs(&[Token::Identififier("hoge".to_owned())]),
        Some((&[] as &[Token], Lhs::Pointer("hoge".to_owned())))
    );
    assert_eq!(
        lhs(&[Token::Star, Token::Identififier("hoge".to_owned())]),
        Some((&[] as &[Token], Lhs::Dereference("hoge".to_owned())))
    );
}

pub fn expression(tokens: &[Token]) -> Option<(&[Token], Expression)> {
    lhs(tokens).and_then(|(tokens, l)| match tokens.first() {
        Some(Token::PlusEq) => rhs(&tokens[1..]).map(|(t, r)| (t, Expression::AssignAdd(l, r))),

        Some(Token::MinusEq) => rhs(&tokens[1..]).map(|(t, r)| (t, Expression::AssignSub(l, r))),

        _ => Some((tokens, Expression::Lhs(l))),
    })
}

#[test]
fn test_expression() {
    assert_eq!(expression(&[]), None);

    assert_eq!(
        expression(&[Token::Identififier("hoge".to_owned())]),
        Some((
            &[] as &[Token],
            Expression::Lhs(Lhs::Pointer("hoge".to_owned()))
        ))
    );
    assert_eq!(
        expression(&[Token::Star, Token::Identififier("hoge".to_owned())]),
        Some((
            &[] as &[Token],
            Expression::Lhs(Lhs::Dereference("hoge".to_owned()))
        ))
    );
    assert_eq!(
        expression(&[
            Token::Identififier("hoge".to_owned()),
            Token::Identififier("hoge".to_owned())
        ]),
        Some((
            &[Token::Identififier("hoge".to_owned())] as &[Token],
            Expression::Lhs(Lhs::Pointer("hoge".to_owned()))
        ))
    );

    assert_eq!(
        expression(&[
            Token::Identififier("hoge".to_owned()),
            Token::PlusEq,
            Token::Integer(123)
        ]),
        Some((
            &[] as &[Token],
            Expression::AssignAdd(Lhs::Pointer("hoge".to_owned()), Rhs::Number(123))
        ))
    );
    assert_eq!(
        expression(&[
            Token::Star,
            Token::Identififier("hoge".to_owned()),
            Token::PlusEq,
            Token::Integer(123)
        ]),
        Some((
            &[] as &[Token],
            Expression::AssignAdd(Lhs::Dereference("hoge".to_owned()), Rhs::Number(123))
        ))
    );

    assert_eq!(
        expression(&[
            Token::Identififier("hoge".to_owned()),
            Token::MinusEq,
            Token::Integer(123)
        ]),
        Some((
            &[] as &[Token],
            Expression::AssignSub(Lhs::Pointer("hoge".to_owned()), Rhs::Number(123))
        ))
    );
    assert_eq!(
        expression(&[
            Token::Star,
            Token::Identififier("hoge".to_owned()),
            Token::MinusEq,
            Token::Integer(123)
        ]),
        Some((
            &[] as &[Token],
            Expression::AssignSub(Lhs::Dereference("hoge".to_owned()), Rhs::Number(123))
        ))
    );

    // TODO:
    // assert_eq!(
    //     expression(&[
    //         Token::Identififier("hoge".to_owned()),
    //         Token::ParenOpen,
    //         Token::ParenClose
    //     ]),
    //     Some((&[] as &[Token], Expression::FunctionCall("hoge".to_owned())))
    // );

    assert_eq!(
        expression(&[
            Token::Identififier("hoge".to_owned()),
            Token::MinusEq,
            Token::Identififier("hoge".to_owned()),
        ]),
        None
    );
}

pub fn statement(tokens: &[Token]) -> Option<(&[Token], Statement)> {
    fn expression_s(tokens: &[Token]) -> Option<(&[Token], Statement)> {
        expression(tokens).and_then(|(t, e)| match t.first() {
            Some(Token::Semi) => Some((&t[1..], Statement::Expression(e))),
            _ => None,
        })
    }

    fn block_s(_tokens: &[Token]) -> Option<(&[Token], Statement)> {
        // TODO
        None
    }

    fn while_s(_tokens: &[Token]) -> Option<(&[Token], Statement)> {
        // TODO
        None
    }

    expression_s(tokens)
        .or_else(|| block_s(tokens))
        .or_else(|| while_s(tokens))
}

#[test]
fn test_statement() {
    assert_eq!(statement(&[]), None);

    assert_eq!(
        statement(&[Token::Identififier("hoge".to_owned()), Token::Semi]),
        Some((
            &[] as &[Token],
            Statement::Expression(Expression::Lhs(Lhs::Pointer("hoge".to_owned())))
        ))
    );
    assert_eq!(
        statement(&[
            Token::Star,
            Token::Identififier("hoge".to_owned()),
            Token::Semi
        ]),
        Some((
            &[] as &[Token],
            Statement::Expression(Expression::Lhs(Lhs::Dereference("hoge".to_owned())))
        ))
    );
    assert_eq!(
        statement(&[
            Token::Identififier("hoge".to_owned()),
            Token::PlusEq,
            Token::Integer(123),
            Token::Semi
        ]),
        Some((
            &[] as &[Token],
            Statement::Expression(Expression::AssignAdd(
                Lhs::Pointer("hoge".to_owned()),
                Rhs::Number(123)
            ))
        ))
    );
}
