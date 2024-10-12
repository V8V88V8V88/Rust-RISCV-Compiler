use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[regex(r"\d+", |lex| lex.slice().parse())]
    Integer(i32),

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Asterisk,

    #[token("/")]
    Slash,

    #[token("=")]
    Equals,

    #[token(">")]
    GreaterThan,

    #[token("<")]
    LessThan,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token(";")]
    Semicolon,

    #[token("let")]
    Let,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    Token::lexer(input).collect()
}
