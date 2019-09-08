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

pub fn tokenize(src: &str) -> Result<Vec<Token>, String> {
    let bytelen = src.len();

    lazy_static! {
        static ref WHITESPACES: Regex = Regex::new(r"^\s+").unwrap();
        static ref COMMENT: Regex = Regex::new(r"(?m)^//.+$").unwrap();
        static ref INTEGER: Regex = Regex::new(r"^[[:digit:]]+\b").unwrap();
        static ref ID_OR_KEY: Regex = Regex::new(r"^[[:alpha:]][[:alnum:]]*\b").unwrap();
    }

    let mut tokens = Vec::new();

    let mut cur = 0;
    while cur < bytelen {
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
                ($pat:expr, $closure:expr) => {
                    if s.starts_with($pat) {
                        $closure();
                        cur += $pat.len();
                        continue;
                    }
                };
            }

            match_re!(WHITESPACES, |_| {});
            match_re!(COMMENT, |_| {});

            match_re!(INTEGER, |m: regex::Match| {
                tokens.push(Token::Integer(m.as_str().parse().unwrap()));
            });

            match_str!("+=", || tokens.push(Token::PlusEq));
            match_str!("-=", || tokens.push(Token::MinusEq));
            match_str!("*", || tokens.push(Token::Star));
            match_str!(";", || tokens.push(Token::Semi));
            match_str!("(", || tokens.push(Token::ParenOpen));
            match_str!(")", || tokens.push(Token::ParenClose));
            match_str!("{", || tokens.push(Token::CurlyOpen));
            match_str!("}", || tokens.push(Token::CurlyClose));

            match_re!(ID_OR_KEY, |m: regex::Match| {
                let t = match m.as_str() {
                    "while" => Token::While,
                    s => Token::Identififier(s.to_string()),
                };
                tokens.push(t);
            });

            return Err(format!(
                "unexpected character: '{}'",
                src.get(cur..).iter().next().unwrap()
            ));
        } else {
            return Err(format!("src.get({}..) returns None", cur));
        }
    }

    Ok(tokens)
}

#[test]
fn test_tokenize() {
    assert_eq!(tokenize("     "), Ok(vec![]));
    assert_eq!(tokenize("while"), Ok(vec![Token::While]));
    assert_eq!(
        tokenize("hoge"),
        Ok(vec![Token::Identififier("hoge".to_owned())])
    );
    assert_eq!(tokenize("123"), Ok(vec![Token::Integer(123)]));
    assert_eq!(tokenize("*"), Ok(vec![Token::Star]));
    assert_eq!(tokenize("+="), Ok(vec![Token::PlusEq]));
    assert_eq!(tokenize("-="), Ok(vec![Token::MinusEq]));
    assert_eq!(tokenize(";"), Ok(vec![Token::Semi]));
    assert_eq!(tokenize("("), Ok(vec![Token::ParenOpen]));
    assert_eq!(tokenize(")"), Ok(vec![Token::ParenClose]));
    assert_eq!(tokenize("{"), Ok(vec![Token::CurlyOpen]));
    assert_eq!(tokenize("}"), Ok(vec![Token::CurlyClose]));

    assert!(tokenize("+").is_err());
    assert!(tokenize("-").is_err());
    assert!(tokenize("/").is_err());
    assert!(tokenize(">").is_err());
    assert!(tokenize("<").is_err());

    assert_eq!(tokenize("// coment"), Ok(vec![]));
    assert_eq!(tokenize("123// coment"), Ok(vec![Token::Integer(123)]));
    assert_eq!(
        tokenize(
            r#"// coment
123"#
        ),
        Ok(vec![Token::Integer(123)])
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
