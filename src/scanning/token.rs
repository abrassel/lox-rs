use std::num::ParseFloatError;

use thiserror::Error;

use crate::scanning::{Scanner, TokenResult};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Identifier(String),
    Literal(LiteralKind),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralKind {
    Str(String),
    Number(f64),
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line_number: usize,
}

impl Token {
    pub fn consume(source: &mut Scanner) -> Option<TokenResult> {
        use TokenType::*;
        let c = source.advance_skipping_whitespace()?;
        let line_number = source.line_number;
        let result: Result<Self, TokenErrorKind> = try {
            let token_type = match c {
                '(' => LeftParen,
                ')' => RightParen,
                '{' => LeftBrace,
                '}' => RightBrace,
                ',' => Comma,
                '.' => Dot,
                '-' => Minus,
                '+' => Plus,
                ';' => SemiColon,
                '*' => Star,
                '!' => {
                    if source.advance_match('=') {
                        BangEqual
                    } else {
                        Bang
                    }
                }
                '=' => {
                    if source.advance_match('=') {
                        EqualEqual
                    } else {
                        Equal
                    }
                }
                '<' => {
                    if source.advance_match('=') {
                        LessEqual
                    } else {
                        Less
                    }
                }
                '>' => {
                    if source.advance_match('=') {
                        GreaterEqual
                    } else {
                        Greater
                    }
                }
                '/' => {
                    if source.advance_match('/') {
                        // discard comments
                        let _ = source.advance_until('\n');
                        return Self::consume(source);
                    } else {
                        Slash
                    }
                }
                '"' => Literal(LiteralKind::Str(source.advance_until('"')?)),
                digit if digit.is_ascii_digit() => {
                    let mut number = digit.into();
                    source.finish_number(&mut number);
                    Literal(LiteralKind::Number(number.parse()?))
                }
                c if c.is_ascii_alphanumeric() => {
                    let mut identifier = c.to_string();
                    source.finish_identifier(&mut identifier);

                    match identifier.as_str() {
                        "and" => TokenType::And,
                        "class" => TokenType::Class,
                        "else" => TokenType::Else,
                        "false" => TokenType::False,
                        "fun" => TokenType::Fun,
                        "for" => TokenType::For,
                        "if" => TokenType::If,
                        "nil" => TokenType::Nil,
                        "or" => TokenType::Or,
                        "print" => TokenType::Print,
                        "return" => TokenType::Return,
                        "super" => TokenType::Super,
                        "this" => TokenType::This,
                        "true" => TokenType::True,
                        "var" => TokenType::Var,
                        "while" => TokenType::While,
                        //todo: intern identifiers
                        _ => Identifier(identifier),
                    }
                }
                other => {
                    return Some(Err(TokenError {
                        line_number,
                        kind: TokenErrorKind::UnexpectedCharacter(other),
                    }));
                }
            };

            Self {
                token_type,
                line_number,
            }
        };

        Some(result.map_err(|kind| TokenError { line_number, kind }))
    }
}

#[derive(Debug, Error)]
pub enum TokenErrorKind {
    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),
    #[error("EOF before expected character: {0}")]
    MissingCharacter(char),
    #[error(transparent)]
    Other(#[from] ParseFloatError),
}

#[derive(Debug)]
pub struct TokenError {
    pub line_number: usize,
    pub kind: TokenErrorKind,
}
