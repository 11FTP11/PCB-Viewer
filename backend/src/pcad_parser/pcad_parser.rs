use std::iter::Peekable;
use std::error::Error;
use std::fmt;
use crate::models::pcb::*;
use super::{
    TokenKind,
    TokenType,
    PcadLexer,
    Token,
    LexerError
};

#[derive(Debug, Clone)]
pub enum ParserError {
    ParseIntError(std::num::ParseIntError),
    ParseFloatError(std::num::ParseFloatError),
    ParseBoolError(String),
    UnexpectedCharacter(String),
    FromUtf8Error(std::string::FromUtf8Error),
    Utf8Error(std::str::Utf8Error),
    LexicalAnalysis(LexerError),
    GeneralError(&'static str),
    StructureError(&'static str),
    UnexpectedToken(String, Token)
}

impl fmt::Display for ParserError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::ParseIntError(e) => write!(f, "Integer parse error: {}", e),
            ParserError::ParseFloatError(e) => write!(f, "Float parse error: {}", e),
            ParserError::ParseBoolError(s) => write!(f, "Boolean parse error: {}", s),
            ParserError::Utf8Error(e) => write!(f, "UTF-8 error: {}", e),
            ParserError::UnexpectedCharacter(e) => write!(f, "Unexpected character '{}'", e),
            ParserError::LexicalAnalysis(e) => write!(f, "Lexer error: {}", e),
            ParserError::GeneralError(e) => write!(f, "Error: {}", e),
            ParserError::StructureError(e) => write!(f, "Structure error: {}", e),
            ParserError::UnexpectedToken(lexeme, token) => write!(f, "Unexpected lexeme: {}, expected {:?} at line {}, column {}", lexeme, token.lexeme(), token.pos.0, token.pos.1 ),
            _ => write!(f, "unexpected error!") 
        }
    }
}

impl Error for ParserError {}

impl From<std::num::ParseIntError> for ParserError {
    fn from(e: std::num::ParseIntError) -> Self {
        ParserError::ParseIntError(e)
    }
}

impl From<std::num::ParseFloatError> for ParserError {
    fn from(e: std::num::ParseFloatError) -> Self {
        ParserError::ParseFloatError(e)
    }
}

impl From<std::str::Utf8Error> for ParserError {
    fn from(e: std::str::Utf8Error) -> Self {
        ParserError::Utf8Error(e)
    }
}

impl From<std::string::FromUtf8Error> for ParserError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        ParserError::Utf8Error(e.utf8_error()) 
    }
}

pub enum Wrapped {
    In,
    Out
}

pub trait PcadDeserialize: Default {
    const TAG: &'static [u8];
    const HAS_INTERNAL_NAME: bool = false;

    fn load_field(&mut self, field_name: &[u8], parser: &mut PcadParser) -> Result<bool, ParserError>;
    fn load_internal_name(&mut self, field_name: &[u8]) {}
}

pub struct PcadParser<'a> {
    lexer: Peekable<PcadLexer<'a>>,
}

impl<'a> PcadParser<'a> {
    pub fn new(lexer: PcadLexer<'a>) -> Self {
        PcadParser {
            lexer: lexer.peekable(),
        }
    }

    pub fn next_token(&mut self) -> Result<Token, ParserError> {
        let token = self.lexer
            .next()
            .transpose()
            .map_err(ParserError::LexicalAnalysis)?
            .ok_or(ParserError::GeneralError("Unexpected end of input"));

        Ok(token?)
    }

    pub fn parse_file_header(&mut self) -> Result<(), ParserError> {
        if self.parse_string()? != "ACCEL_ASCII" {
            return Err(ParserError::StructureError("Not an ACCEL ASCII format"));
        }
        self.expect_token(TokenKind::String);
        Ok(())
    }

    pub fn parse_spec(&mut self) -> Result<PcbSpec, ParserError> {
        self.parse_file_header()?;
        Ok(
            PcbSpec {
                ascii_header: self.parse_common_struct::<AsciiHeader>(Wrapped::Out)?,
                library: self.parse_common_struct::<Library>(Wrapped::Out)?,
                netlist: self.parse_common_struct::<Netlist>(Wrapped::Out)?,
                pcb_design: self.parse_common_struct::<PcbDesign>(Wrapped::Out)?
            }
        )
    }

    pub fn raw_parse_u32(lexeme: &[u8]) -> Result<u32, ParserError> {
        Ok(std::str::from_utf8(lexeme)?.parse::<u32>()?)
    }

    pub fn raw_parse_u64(lexeme: &[u8]) -> Result<u64, ParserError> {
        Ok(std::str::from_utf8(lexeme)?.parse::<u64>()?)
    }

    pub fn raw_parse_i32(lexeme: &[u8]) -> Result<i32, ParserError> {
        Ok(std::str::from_utf8(lexeme)?.parse::<i32>()?) 
    }

    pub fn raw_parse_i64(lexeme: &[u8]) -> Result<i64, ParserError> {
        Ok(std::str::from_utf8(lexeme)?.parse::<i64>()?)
    }

    pub fn raw_parse_f32(lexeme: &[u8]) -> Result<f32, ParserError> {
        Ok(std::str::from_utf8(lexeme)?.parse::<f32>()?)
    }

    pub fn raw_parse_f64(lexeme: &[u8]) -> Result<f64, ParserError> {
        Ok(std::str::from_utf8(lexeme)?.parse::<f64>()?)
    }

    pub fn raw_parse_bool(lexeme: &[u8]) -> Result<bool, ParserError> {
        let s = std::str::from_utf8(lexeme)?;
        match s.to_ascii_lowercase().as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(ParserError::GeneralError("Invalid boolean value")),
        }
    }

    pub fn raw_parse_string(lexeme: &[u8]) -> String {
        String::from_utf8_lossy(lexeme).into_owned() 
    }

    pub fn parse_u32(&mut self) -> Result<u32, ParserError> {
        let token = self.expect_token(TokenKind::Integer)?;
        if let TokenType::Integer(token) = token.token {
            return Ok(std::str::from_utf8(&token)?.parse::<u32>()?);
        }
        Err(ParserError::GeneralError("Expected integer"))
    }

    pub fn parse_i32(&mut self) -> Result<i32, ParserError> {
        let token = self.expect_token(TokenKind::Integer)?;
        if let TokenType::Integer(token) = token.token {
            return Ok(std::str::from_utf8(&token)?.parse::<i32>()?);
        }
        Err(ParserError::GeneralError("Expected integer"))
    }

    pub fn parse_f32(&mut self) -> Result<f32, ParserError> {
        let token = self.expect_token(TokenKind::Float)?;
        if let TokenType::Float(token) = token.token {
            return Ok(std::str::from_utf8(&token)?.parse::<f32>()?);
        }
        Err(ParserError::GeneralError("Expected float"))
    }
    
    pub fn parse_number64(&mut self) -> Result<f64, ParserError> {
        let token = self.next_token()?;
        match token.token {
            TokenType::Float(bytes) => Ok(std::str::from_utf8(&bytes)?.parse::<f64>()?),
            TokenType::Integer(bytes) => Ok(std::str::from_utf8(&bytes)?.parse::<f64>()?),
            _ => Err(ParserError::GeneralError("Expected float"))
        }
    }

    pub fn parse_f64(&mut self) -> Result<f64, ParserError> {
        let token = self.expect_token(TokenKind::Float)?;
        if let TokenType::Float(token) = token.token {
            return Ok(std::str::from_utf8(&token)?.parse::<f64>()?);
        }
        Err(ParserError::GeneralError("Expected float"))
    }

    pub fn parse_string(&mut self) -> Result<String, ParserError> {
        let token = self.next_token()?;
        match &token.token {
            TokenType::String(bytes) | TokenType::Symbol(bytes) => {
                Ok(String::from_utf8_lossy(bytes).into_owned())
            }
            _ => Err(ParserError::GeneralError("Expected string or symbol")),
        }
    }

    pub fn parse_bool(&mut self) -> Result<bool, ParserError> {
        let token = self.next_token()?;
        match &token.token {
            TokenType::BoolTrue => Ok(true),
            TokenType::BoolFalse => Ok(false),
            _ => Err(ParserError::GeneralError("Expected boolean"))
        }

    }

    pub fn expect_token(&mut self, target: TokenKind) -> Result<Token, ParserError> {
        let tok = self.next_token()?;
        if matches!(tok.token.kind(), target) {
            Ok(tok)
        } else {
            Err(ParserError::UnexpectedCharacter(format!("{:?}", tok)))
        }
    }

    pub fn peek_token_type(&mut self) -> Option<TokenType> {
        match self.lexer.peek() {
            Some(Ok(Token { token, .. })) => Some(token.clone()),
            _ => None,
        }
    }

    pub fn parse_field<T,F>(&mut self, value_handler_fn: F) -> Result<T, ParserError>
    where 
        F: FnOnce(&mut PcadParser<'a>) -> Result<T, ParserError>,
    {
        let result = value_handler_fn(self);
        self.expect_token(TokenKind::CloseParen)?;
        return result;
    }

    pub fn expect_token_bytes(&mut self, target: &[u8]) -> Result<Token, ParserError> {
        let token = self.next_token()?;
        let recieved = token.lexeme();
        if recieved == target {
            return Ok(token);
        }

        return Err(ParserError::UnexpectedToken(String::from_utf8_lossy(recieved).into(), token));
    }

    pub fn skip_field_value(&mut self) -> Result<(), ParserError> {
        while !matches!(self.peek_token_type(), Some(TokenType::CloseParen)) {
            self.next_token()?;
        }
        self.expect_token(TokenKind::CloseParen)?;
        Ok(())
    }


    pub fn parse_attr_struct(&mut self, wrap_mode: Wrapped) -> Result<Attr, ParserError> 
    {
        let mut builder = Attr::default();
        match wrap_mode {
            Wrapped::Out => {
                self.expect_token(TokenKind::OpenParen)?;
                self.expect_token_bytes(b"attr")?;
            },
            Wrapped::In => {}
        }


        let tok = self.expect_token(TokenKind::String)?;
        builder.label = String::from_utf8_lossy(tok.lexeme()).into_owned();
        let tok = self.expect_token(TokenKind::String)?;
        builder.name = String::from_utf8_lossy(tok.lexeme()).into_owned();

        loop {
            if matches!(self.peek_token_type(), Some(TokenType::CloseParen)) {
                self.expect_token(TokenKind::CloseParen)?;
                break;
            }

            self.expect_token(TokenKind::OpenParen)?;
            let field_name = self.next_token()?;
            match field_name.token.kind() {
                TokenKind::Symbol => {
                    if !builder.load_field(&field_name.lexeme(), self)? {
                        println!("FALSE");
                        self.skip_field_value()?;
                    }
                },
                _ => {
                    return Err(ParserError::GeneralError("Expected field name (symbol)"));
                }
            }
        }

        Ok(builder)
    }

    pub fn parse_seq_struct<T>(&mut self, wrap_mode: Wrapped) -> Result<T, ParserError>
    where
        T: PcadDeserialize + Default,
    {

        let mut builder = T::default();
        match wrap_mode {
            Wrapped::Out => {
                self.expect_token(TokenKind::OpenParen)?;
                self.expect_token_bytes(T::TAG)?;
            },
            Wrapped::In => {}
        }

        if T::HAS_INTERNAL_NAME {
            let tok = self.expect_token(TokenKind::String)?;
            builder.load_internal_name(tok.lexeme());
        }

        loop {
            if matches!(self.peek_token_type(), Some(TokenType::CloseParen)) {
                self.expect_token(TokenKind::CloseParen)?;
                break;
            }

            let value = self.next_token()?;
            if !builder.load_field(&value.lexeme(), self)? {
                println!("FALSE");
                self.skip_field_value()?;
            }
        }

        Ok(builder)
    }


    pub fn parse_common_struct<T>(&mut self, wrap_mode: Wrapped) -> Result<T, ParserError> 
    where 
        T: PcadDeserialize + Default,
    {
        
        let mut builder = T::default();
        match wrap_mode {
            Wrapped::Out => {
                self.expect_token(TokenKind::OpenParen)?;
                self.expect_token_bytes(T::TAG)?;
            },
            Wrapped::In => {}
        }

        if T::HAS_INTERNAL_NAME {
            let tok = self.expect_token(TokenKind::String)?;
            builder.load_internal_name(tok.lexeme());
        }

        loop {
            if matches!(self.peek_token_type(), Some(TokenType::CloseParen)) {
                self.expect_token(TokenKind::CloseParen)?;
                break;
            }

            self.expect_token(TokenKind::OpenParen)?;
            let field_name = self.next_token()?;
            match field_name.token.kind() {
                TokenKind::Symbol => {
                    if !builder.load_field(&field_name.lexeme(), self)? {
                        println!("FALSE");
                        self.skip_field_value()?;
                    }
                },
                _ => {
                    return Err(ParserError::GeneralError("Expected field name (symbol)"));
                }
            }
        }

        Ok(builder)
    }
}

