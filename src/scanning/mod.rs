use std::str::Chars;

use itertools::{Itertools, MultiPeek};

use crate::scanning::token::{Token, TokenError, TokenErrorKind};

pub mod token;

pub type TokenResult = Result<Token, TokenError>;

pub fn scan_tokens(source: &str) -> Scanner {
    Scanner {
        source: source.chars().multipeek(),
        line_number: 0,
    }
}

pub struct Scanner<'a> {
    source: MultiPeek<Chars<'a>>,
    line_number: usize,
}

impl Scanner<'_> {
    pub(crate) fn advance(&mut self) -> Option<char> {
        let char = self.source.next()?;
        if char == '\n' {
            self.line_number += 1;
        }

        Some(char)
    }

    pub(crate) fn advance_match(&mut self, m: char) -> bool {
        if self.source.peek().contains(&&m) {
            self.advance();
            true
        } else {
            self.source.reset_peek();
            false
        }
    }

    pub(crate) fn advance_skipping_whitespace(&mut self) -> Option<char> {
        while let Some(c) = self.advance() {
            if !c.is_ascii_whitespace() {
                return Some(c);
            }
        }

        None
    }

    pub(crate) fn advance_until(&mut self, c: char) -> Result<String, TokenErrorKind> {
        let mut buf = String::new();
        while let Some(char) = self.advance() {
            if char == c {
                return Ok(buf);
            }

            buf.push(char);
        }

        Err(TokenErrorKind::MissingCharacter(c))
    }

    pub(crate) fn finish_number(&mut self, buf: &mut String) {
        let mut period_seen = false;
        // todo: uggo
        while let Some(&next) = self.source.peek() {
            if next.is_ascii_digit() {
                buf.push(self.advance().unwrap());
            } else if !period_seen
                && next == '.'
                && self.source.peek().map_or(false, char::is_ascii_digit)
            {
                period_seen = true;
                buf.push(self.advance().unwrap());
            } else {
                break;
            }
        }
    }

    pub(crate) fn finish_identifier(&mut self, buf: &mut String) {
        // todo: uggo
        while let Some(&next) = self.source.peek() {
            if next.is_ascii_alphanumeric() {
                buf.push(self.advance().unwrap());
            } else {
                break;
            }
        }
    }
}

impl Iterator for Scanner<'_> {
    type Item = TokenResult;

    fn next(&mut self) -> Option<Self::Item> {
        Token::consume(self)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::scan_tokens;
    use crate::scanning::{
        token::{LiteralKind::*, Token, TokenType, TokenType::*},
        TokenError,
    };

    fn tokenify(toks: Vec<Vec<TokenType>>) -> Vec<Token> {
        let mut tokens = vec![];
        for (line_number, toks) in toks.into_iter().enumerate() {
            tokens.extend(toks.into_iter().map(|token_type| Token {
                line_number,
                token_type,
            }))
        }

        tokens
    }

    #[test]
    fn test_math() -> Result<(), TokenError> {
        let line = "(3*5+(4/5))-3.58";
        let parsed = scan_tokens(line).collect::<Result<Vec<_>, _>>()?;
        let expected: Vec<_> = tokenify(vec![vec![
            LeftParen,
            Literal(Number(3.0)),
            Star,
            Literal(Number(5.0)),
            Plus,
            LeftParen,
            Literal(Number(4.0)),
            Slash,
            Literal(Number(5.0)),
            RightParen,
            RightParen,
            Minus,
            Literal(Number(3.58)),
        ]]);

        assert_eq!(parsed, expected);

        Ok(())
    }

    #[test]
    fn test_hello_world() -> Result<(), TokenError> {
        let line = "var test = \"hello world\";

            if (x == 3) {
                print test;
            }
            else if (x < 4 and x <= 3) {

                print test + \"goodbye\";
            }
        ";
        let parsed = scan_tokens(line).collect::<Result<Vec<_>, _>>()?;
        let expected = tokenify(vec![
            vec![
                Var,
                Identifier("test".into()),
                Equal,
                Literal(Str("hello world".into())),
                SemiColon,
            ],
            vec![],
            vec![
                If,
                LeftParen,
                Identifier("x".into()),
                EqualEqual,
                Literal(Number(3.0)),
                RightParen,
                LeftBrace,
            ],
            vec![Print, Identifier("test".into()), SemiColon],
            vec![RightBrace],
            vec![
                Else,
                If,
                LeftParen,
                Identifier("x".into()),
                Less,
                Literal(Number(4.0)),
                And,
                Identifier("x".into()),
                LessEqual,
                Literal(Number(3.0)),
                RightParen,
                LeftBrace,
            ],
            vec![],
            vec![
                Print,
                Identifier("test".into()),
                Plus,
                Literal(Str("goodbye".into())),
                SemiColon,
            ],
            vec![RightBrace],
        ]);

        assert_eq!(parsed, expected);

        Ok(())
    }
}
