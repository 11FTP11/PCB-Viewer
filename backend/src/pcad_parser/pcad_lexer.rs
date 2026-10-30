use std::error::Error;
use std::fmt;

pub struct PcadLexer<'a> {
    content: &'a [u8],
    line: usize,
    cursor: usize,
    bol: usize,
}

impl<'a> PcadLexer<'a> {
    pub fn new(content: &'a [u8]) -> PcadLexer {
        PcadLexer {
            content,
            cursor: 0,
            line: 1,
            bol: 0,
        }
    }

    pub fn peek(&self) -> Option<u8> {
        self.content.get(self.cursor).copied()
    }

    pub fn consume(&mut self, len: usize) {
        for _ in 0..len {
            if self.cursor >= self.content.len() {
                break;
            }
            let sym = self.content[self.cursor];
            self.cursor += 1;
            if sym == b'\n' {
                self.line += 1;
                self.bol = self.cursor;
            }
        }
    }

    pub fn trim_left(&mut self) {
        while self.cursor < self.content.len() && self.content[self.cursor].is_ascii_whitespace() {
            self.consume(1);
        }
    }

    pub fn get_column(&self) -> usize {
        self.cursor - self.bol + 1
    }

    pub fn read_until(&mut self, predicate_fn: impl Fn(u8) -> bool) {
        while self.cursor < self.content.len() && predicate_fn(self.content[self.cursor]) {
            self.consume(1);
        }
    }

    pub fn is_keyword(&self, word: &[u8]) -> Option<TokenType> {
        match word {
            b"True" => Some(TokenType::BoolTrue),
            b"False" => Some(TokenType::BoolFalse),
            _ => None,
        }
    }

    fn consume_exponent(&mut self) {
        if let Some(b'+') | Some(b'-') = self.peek() {
            self.consume(1);
        }
        self.read_until(|byte| byte.is_ascii_digit());
    }

    pub fn analyze_bytes(
        &mut self,
        start_pos: usize,
        start_line: usize,
        start_column: usize,
    ) -> Result<TokenType, LexerError> {
        match self.peek() {
            Some(b'(') => {
                self.consume(1);
                Ok(TokenType::OpenParen)
            }
            Some(b')') => {
                self.consume(1);
                Ok(TokenType::CloseParen)
            }
            Some(b'"') => {
                self.consume(1);
                loop {
                    match self.peek() {
                        Some(b'"') => {
                            self.consume(1);
                            return Ok(TokenType::String(self.content[start_pos + 1..self.cursor - 1].to_vec()));
                        }
                        Some(b'\n' | b'\r') => {
                            return Err(LexerError::new(
                                (self.line, self.get_column()),
                                "unterminated string literal.",
                            ));
                        }
                        None => {
                            return Err(LexerError::new(
                                (self.line, self.get_column()),
                                "unterminated string literal.",
                            ));
                        }
                        _ => {
                            self.consume(1);
                        }
                    }
                }
            }
            Some(b'A'..=b'Z' | b'a'..=b'z') => {
                let start = self.cursor;
                self.read_until(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'));
                let word = &self.content[start..self.cursor];
                if let Some(token_type) = self.is_keyword(word) {
                    Ok(token_type)
                } else {
                    Ok(TokenType::Symbol(word.to_vec()))
                }
            }
            Some(b'0'..=b'9' | b'-') => {
                let start = self.cursor;
                if self.peek() == Some(b'-') {
                    self.consume(1);
                }
                if self.peek().map_or(false, |b| b.is_ascii_digit()) {
                    self.read_until(|byte| byte.is_ascii_digit());
                    match self.peek() {
                        Some(b'.') => {
                            self.consume(1);
                            self.read_until(|byte| byte.is_ascii_digit());
                            if matches!(self.peek(), Some(b'e') | Some(b'E')) {
                                self.consume(1);
                                self.consume_exponent();
                            }
                            if matches!(self.peek(), Some(b' ') | Some(b')') | None) {
                                Ok(TokenType::Float(self.content[start..self.cursor].to_vec()))
                            } else {
                                Err(LexerError::new((self.line, self.get_column()), "invalid float number"))
                            }
                        }
                        Some(b'e') | Some(b'E') => {
                            self.consume(1);
                            self.consume_exponent();
                            Ok(TokenType::Integer(self.content[start..self.cursor].to_vec()))
                        }
                        Some(b' ') | Some(b')') | None => {
                            Ok(TokenType::Integer(self.content[start..self.cursor].to_vec()))
                        }
                        _ => Err(LexerError::new((self.line, self.get_column()), "Invalid number.")),
                    }
                } else {
                    Err(LexerError::new((self.line, self.get_column()), "Invalid number."))
                }
            }
            None => Ok(TokenType::EOF),
            Some(_) => Err(LexerError::new(
                (self.line, self.get_column()),
                "Unexpected character",
            )),
        }
    }
}

impl<'a> Iterator for PcadLexer<'a> {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.trim_left();

        if self.cursor >= self.content.len() {
            return None;
        }

        let start_pos = self.cursor;
        let start_line = self.line;
        let start_column = self.get_column();

        match self.analyze_bytes(start_pos, start_line, start_column) {
            Ok(token_type) => Some(Ok(Token {
                pos: (start_line, start_column),
                token: token_type,
            })),
            Err(e) => Some(Err(e)),
        }
    }
}



#[derive(Debug, Clone)]
pub struct LexerError {
    pos: (usize, usize),
    message: &'static str 
}

impl LexerError {
    pub fn new(pos: (usize, usize), message: &'static str) -> Self {
        LexerError {
            pos,
            message
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "(line: {}, column: {}, message: {})", 
            self.pos.0, self.pos.1, self.message
        )
    }
}

impl Error for LexerError {}

#[derive(Debug, Clone)]
pub struct Token {
    pub pos: (usize, usize),
    pub token: TokenType,
}

impl Token {
    pub fn lexeme(&self) -> &[u8] {
        match &self.token {
            TokenType::OpenParen => b"(",
            TokenType::CloseParen => b")",
            TokenType::BoolTrue => b"True",
            TokenType::BoolFalse => b"False",
            TokenType::Symbol(l) => l.as_slice(),
            TokenType::Keyword(l) => l.as_slice(),
            TokenType::String(l) => l.as_slice(),
            TokenType::Integer(l) => l.as_slice(),
            TokenType::Float(l) => l.as_slice(),
            TokenType::EOF => b"",
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    OpenParen,
    CloseParen,
    BoolTrue,
    BoolFalse,
    Symbol(Vec<u8>),
    Keyword(Vec<u8>),
    String(Vec<u8>),
    Integer(Vec<u8>),
    Float(Vec<u8>),
    EOF
}

impl TokenType {
    pub fn kind(&self) -> TokenKind {
        match self {
            TokenType::OpenParen => TokenKind::OpenParen,
            TokenType::CloseParen => TokenKind::CloseParen,
            TokenType::BoolTrue => TokenKind::BoolTrue,
            TokenType::BoolFalse => TokenKind::BoolFalse,
            TokenType::Symbol(_) => TokenKind::Symbol,
            TokenType::Keyword(_) => TokenKind::Keyword,
            TokenType::String(_) => TokenKind::String,
            TokenType::Integer(_) => TokenKind::Integer,
            TokenType::Float(_) => TokenKind::Float,
            TokenType::EOF => TokenKind::EOF
        }
    }
}

#[derive(Debug)]
pub enum TokenKind {
    OpenParen,
    CloseParen,
    BoolTrue,
    BoolFalse,
    Symbol,
    Keyword,
    String,
    Integer,
    Float,
    EOF
} 
