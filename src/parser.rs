use crate::lexer::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Node {
    Character(char),
    Empty,
    Star(Box<Node>),
    Union(Box<Node>, Box<Node>),
    Concat(Box<Node>, Box<Node>),
}

fn error_msg(expected: &[Token], actual: Token) -> String {
    let expected = expected
        .iter()
        .map(|token| format!("'{}'", token))
        .collect::<Vec<_>>()
        .join(", ");
    let actual = match actual {
        Token::Character(char) => format!("'{}'", char),
        _ => format!("'{}'", actual),
    };
    format!("Expected one of [{}], found {}", expected, actual)
}

type Result<T> = std::result::Result<T, String>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    look: Token,
}

impl Parser<'_> {
    pub fn new(mut lexer: Lexer) -> Parser {
        let node = lexer.scan();
        Parser { lexer, look: node }
    }

    pub fn parse(&mut self) -> Result<Node> {
        self.expression()
    }

    fn match_next(&mut self, token: Token) -> Result<()> {
        match &self.look {
            look if *look == token => {
                self.look = self.lexer.scan();
                Ok(())
            }
            other => Err(error_msg(&[token], *other)),
        }
    }

    /// expression := sub_expression EOF
    fn expression(&mut self) -> Result<Node> {
        let expression = self.sub_expression();
        self.match_next(Token::EndOfFile)?;
        expression
    }

    /// sub_expression := sequence '|' sub_expression | sequence
    fn sub_expression(&mut self) -> Result<Node> {
        let sequence = self.sequence();
        match &self.look {
            Token::UnionOperator => {
                self.match_next(Token::UnionOperator)?;
                Ok(Node::Union(
                    Box::new(sequence?),
                    Box::new(self.sub_expression()?),
                ))
            }
            _ => sequence,
        }
    }

    /// sequence := sub_sequence | ''
    fn sequence(&mut self) -> Result<Node> {
        match &self.look {
            Token::LeftParen | Token::Character(_) => self.sub_sequence(),
            _ => Ok(Node::Empty),
        }
    }

    /// sub_sequence := star sub_sequence | star
    fn sub_sequence(&mut self) -> Result<Node> {
        let star = self.star();
        match &self.look {
            Token::LeftParen | Token::Character(_) => Ok(Node::Concat(
                Box::new(star?),
                Box::new(self.sub_sequence()?),
            )),
            _ => star,
        }
    }

    /// star := factor '*' | factor
    fn star(&mut self) -> Result<Node> {
        let factor = self.factor();
        match &self.look {
            Token::StarOperator => {
                self.match_next(Token::StarOperator)?;
                Ok(Node::Star(Box::new(factor?)))
            }
            _ => factor,
        }
    }

    /// factor := '(' subexpr ')' | Character
    fn factor(&mut self) -> Result<Node> {
        match &self.look {
            Token::LeftParen => {
                self.match_next(Token::LeftParen)?;
                let node = self.sub_expression();
                self.match_next(Token::RightParen)?;
                node
            }
            Token::Character(char) => {
                let node = Node::Character(*char);
                self.match_next(Token::Character(*char))?;
                Ok(node)
            }
            other => Err(error_msg(
                &[Token::LeftParen, Token::Character('_')],
                *other,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::*;
    use crate::parser::*;

    #[test]
    fn expression() {
        let mut parser = Parser::new(Lexer::new(r"a|(bc)*"));
        assert_eq!(
            parser.expression(),
            Ok(Node::Union(
                Box::new(Node::Character('a')),
                Box::new(Node::Star(Box::new(Node::Concat(
                    Box::new(Node::Character('b')),
                    Box::new(Node::Character('c'))
                ))))
            ))
        );
    }

    #[test]
    fn expression2() {
        let mut parser = Parser::new(Lexer::new(r"a|"));
        assert_eq!(
            parser.expression(),
            Ok(Node::Union(
                Box::new(Node::Character('a')),
                Box::new(Node::Empty)
            ))
        );
    }

    #[test]
    fn fail() {
        let mut parser1 = Parser::new(Lexer::new(r"a("));
        let mut parser2 = Parser::new(Lexer::new(r"a)"));
        assert!(parser1.expression().is_err());
        assert!(parser2.expression().is_err());
    }
}
