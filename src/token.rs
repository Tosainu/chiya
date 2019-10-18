use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Token {
    Integer(i32),
    Identififier(String),

    // Keywords
    While, // 'while'

    // Punctuation symbols
    Star,       // '*'
    PlusEq,     // '+='
    MinusEq,    // '-='
    Semi,       // ';'
    ParenOpen,  // '('
    ParenClose, // ')'
    CurlyOpen,  // '{'
    CurlyClose, // '}'
}

#[derive(Debug, PartialEq, failure::Fail)]
pub enum TokenizerError {
    #[fail(display = "unexpected character: '{}'", character)]
    UnexpectedCharacter { character: char },

    #[fail(display = "input error")]
    InputError,
}

pub fn tokenize(src: &str) -> Result<Vec<Token>, TokenizerError> {
    let mut tokens = Vec::new();
    let mut cur = 0;
    while cur < src.len() {
        if let Some(s) = src.get(cur..) {
            macro_rules! match_re {
                ($re:expr, $closure:expr) => {
                    if let Some(m) = $re.find(s) {
                        $closure(m);
                        cur += m.end();
                        continue;
                    }
                };
            }

            macro_rules! match_str {
                ($pat:expr, $e:expr) => {
                    if s.starts_with($pat) {
                        $e;
                        cur += $pat.len();
                        continue;
                    }
                };
            }

            lazy_static! {
                static ref WHITESPACES: Regex = Regex::new(r"^\s+").unwrap();
                static ref COMMENT: Regex = Regex::new(r"^(?m://.+$)").unwrap();
                static ref INTEGER: Regex = Regex::new(r"^-?\d+\b").unwrap();
                static ref ID_OR_KEY: Regex =
                    Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]*|_[a-zA-Z0-9_]+\b").unwrap();
            }

            match_re!(WHITESPACES, |_| {});
            match_re!(COMMENT, |_| {});

            match_re!(INTEGER, |m: regex::Match| {
                tokens.push(Token::Integer(m.as_str().parse().unwrap()));
            });

            match_str!("+=", tokens.push(Token::PlusEq));
            match_str!("-=", tokens.push(Token::MinusEq));
            match_str!("*", tokens.push(Token::Star));
            match_str!(";", tokens.push(Token::Semi));
            match_str!("(", tokens.push(Token::ParenOpen));
            match_str!(")", tokens.push(Token::ParenClose));
            match_str!("{", tokens.push(Token::CurlyOpen));
            match_str!("}", tokens.push(Token::CurlyClose));

            match_re!(ID_OR_KEY, |m: regex::Match| {
                let t = match m.as_str() {
                    "while" => Token::While,
                    s => Token::Identififier(s.to_string()),
                };
                tokens.push(t);
            });

            return Err(TokenizerError::UnexpectedCharacter {
                character: s.chars().next().unwrap(),
            });
        } else {
            return Err(TokenizerError::InputError);
        }
    }

    Ok(tokens)
}

#[test]
fn test_tokenize() {
    assert_eq!(tokenize("     "), Ok(vec![]));
    assert_eq!(tokenize("123"), Ok(vec![Token::Integer(123)]));
    assert_eq!(tokenize("-123"), Ok(vec![Token::Integer(-123)]));

    assert_eq!(
        tokenize("123 123"),
        Ok(vec![Token::Integer(123), Token::Integer(123)])
    );
    assert_eq!(tokenize("*"), Ok(vec![Token::Star]));
    assert_eq!(tokenize("+="), Ok(vec![Token::PlusEq]));
    assert_eq!(tokenize("-="), Ok(vec![Token::MinusEq]));
    assert_eq!(tokenize(";"), Ok(vec![Token::Semi]));
    assert_eq!(tokenize("("), Ok(vec![Token::ParenOpen]));
    assert_eq!(tokenize(")"), Ok(vec![Token::ParenClose]));
    assert_eq!(tokenize("{"), Ok(vec![Token::CurlyOpen]));
    assert_eq!(tokenize("}"), Ok(vec![Token::CurlyClose]));

    assert_eq!(tokenize("while"), Ok(vec![Token::While]));
    assert_eq!(
        tokenize("hoge"),
        Ok(vec![Token::Identififier("hoge".to_owned())])
    );
    assert_eq!(
        tokenize("hoge_fuga"),
        Ok(vec![Token::Identififier("hoge_fuga".to_owned())])
    );
    assert_eq!(
        tokenize("hoge123"),
        Ok(vec![Token::Identififier("hoge123".to_owned())])
    );
    assert!(tokenize("_").is_err());
    assert_eq!(
        tokenize("_hoge"),
        Ok(vec![Token::Identififier("_hoge".to_owned())])
    );

    assert!(tokenize("+").is_err());
    assert!(tokenize("-").is_err());
    assert!(tokenize("/").is_err());
    assert!(tokenize(">").is_err());
    assert!(tokenize("<").is_err());

    assert_eq!(tokenize("// coment"), Ok(vec![]));
    assert_eq!(tokenize("123// coment"), Ok(vec![Token::Integer(123)]));
    assert_eq!(
        tokenize(
            r#"123
// coment
456"#
        ),
        Ok(vec![Token::Integer(123), Token::Integer(456)])
    );

    assert_eq!(
        tokenize("foo();"),
        Ok(vec![
            Token::Identififier("foo".to_owned()),
            Token::ParenOpen,
            Token::ParenClose,
            Token::Semi
        ])
    );

    assert_eq!(
        tokenize("while *ptr {\nptr -= 1;\n}"),
        Ok(vec![
            Token::While,
            Token::Star,
            Token::Identififier("ptr".to_owned()),
            Token::CurlyOpen,
            Token::Identififier("ptr".to_owned()),
            Token::MinusEq,
            Token::Integer(1),
            Token::Semi,
            Token::CurlyClose,
        ])
    );
}
