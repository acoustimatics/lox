/// Enumerates all kinds of tokens.
#[derive(Debug)]
pub enum TokenKind {
    And,
    Bang,
    BangEqual,
    Class,
    Comma,
    Dot,
    Else,
    Eof,
    Equal,
    EqualEqual,
    False,
    For,
    Fun,
    Greater,
    GreaterEqual,
    Identifier,
    If,
    LeftBrace,
    LeftParen,
    Less,
    LessEqual,
    Minus,
    Nil,
    Number,
    Or,
    Plus,
    Print,
    Return,
    RightBrace,
    RightParen,
    Semicolon,
    Slash,
    Star,
    String,
    Super,
    This,
    True,
    Var,
    While,
}

/// Represents a token from a lox source text.
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: &str, line: usize) -> Token {
        let lexeme = String::from(lexeme);
        Token { kind, lexeme, line }
    }
}

/// Holds the state of a lex scanner.
struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    /// Constructs a new lox scanner that is ready to start scanning the given
    /// lox source.
    fn new<'b>(source: &'b str) -> Scanner<'b> {
        Scanner { source, start: 0, current: 0, line: 1 }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Token {
        unimplemented!()
    }
}

pub fn scan_tokens(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut scanner = Scanner::new(source);

    while !scanner.is_at_end() {
        // We are at the beginning of the next lexeme.
        scanner.start = scanner.current;
        let token = scanner.scan_token();
        tokens.push(token);
    }

    tokens.push(Token::new(TokenKind::Eof, "", scanner.line));

    tokens
}
