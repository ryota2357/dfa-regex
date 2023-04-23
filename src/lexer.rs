use std::fmt::Display;
use std::str::Chars;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token {
    Character(char),
    UnionOperator,
    StarOperator,
    LeftParen,
    RightParen,
    EndOfFile,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Token::Character(_) => "Character",
            Token::UnionOperator => "|",
            Token::StarOperator => "*",
            Token::LeftParen => "(",
            Token::RightParen => ")",
            Token::EndOfFile => "EOF",
        };
        write!(f, "{}", str)
    }
}

pub struct Lexer<'a> {
    string: Chars<'a>,
}

impl Lexer<'_> {
    pub fn new(string: &str) -> Lexer {
        Lexer {
            string: string.chars(),
        }
    }

    pub fn scan(&mut self) -> Token {
        let Some(char) = self.string.next() else {
            return Token::EndOfFile
        };
        match char {
            '\\' => Token::Character(self.string.next().unwrap()),
            '|' => Token::UnionOperator,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '*' => Token::StarOperator,
            _ => Token::Character(char),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::*;

    #[test]
    fn scan() {
        let mut lexer = Lexer::new(r"a|(bc)*");
        assert_eq!(lexer.scan(), Token::Character('a'));
        assert_eq!(lexer.scan(), Token::UnionOperator);
        assert_eq!(lexer.scan(), Token::LeftParen);
        assert_eq!(lexer.scan(), Token::Character('b'));
        assert_eq!(lexer.scan(), Token::Character('c'));
        assert_eq!(lexer.scan(), Token::RightParen);
        assert_eq!(lexer.scan(), Token::StarOperator);
        assert_eq!(lexer.scan(), Token::EndOfFile);
    }

    #[test]
    fn scan_with_escape() {
        let mut lexer = Lexer::new(r"a|\|\\(\)");
        assert_eq!(lexer.scan(), Token::Character('a'));
        assert_eq!(lexer.scan(), Token::UnionOperator);
        assert_eq!(lexer.scan(), Token::Character('|'));
        assert_eq!(lexer.scan(), Token::Character('\\'));
        assert_eq!(lexer.scan(), Token::LeftParen);
        assert_eq!(lexer.scan(), Token::Character(')'));
        assert_eq!(lexer.scan(), Token::EndOfFile);
    }

    #[test]
    fn with_empty() {
        let mut lexer = Lexer::new(r#""#);
        assert_eq!(lexer.scan(), Token::EndOfFile);
    }
}
