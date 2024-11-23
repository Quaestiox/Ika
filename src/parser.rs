use crate::lexer::{Token,TokenType, Error};
use std::collections::HashMap;


// 定义 AST 节点
#[derive(Debug, Clone)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    Literal(String),
    Identifier(String),
}



// 定义解析器结构体
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn peek(&self) -> Result<&Token, Error> {
        if self.current < self.tokens.len() {
            Ok(&self.tokens[self.current])
        } else {
            Err(Error::ParserErr)
        }

    }
    
    fn advance(&mut self) -> Result<&Token, Error> {
        if self.current < self.tokens.len() {
            self.current += 1;
            Ok(&self.tokens[self.current-1])
        } else {
            Err(Error::ParserErr)
        }

    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn peek(){
        let list = [
            Token{
                token_type: TokenType::KEYWORD,
                value:String::from("i32"),    
            },
            Token{
                token_type: TokenType::KEYWORD,
                value:String::from("i32"),    
            }
        ];
        let tokens = Vec::from(list);
        let parser = Parser::new(tokens.clone());
        let first = parser.peek().unwrap();
        let second = parser.peek().unwrap();
        assert_eq!(tokens[0], *first);
        assert_eq!(tokens[1], *second);
    }

    #[test]
    fn peek_overlist(){
        let list = [
            Token{
                token_type: TokenType::KEYWORD,
                value:String::from("i32"),    
            },
            Token{
                token_type: TokenType::KEYWORD,
                value:String::from("i32"),    
            }
        ];
        let tokens = Vec::from(list);
        let mut parser = Parser::new(tokens.clone());
        parser.current = 2;
        let third = parser.peek();

        assert_eq!(third, Err(Error::ParserErr));
        
    }

    #[test]
    fn advance(){
        let list = [
            Token{
                token_type: TokenType::KEYWORD,
                value:String::from("i32"),    
            },
            Token{
                token_type: TokenType::KEYWORD,
                value:String::from("i32"),    
            }
        ];
        let tokens = Vec::from(list);
        let mut parser = Parser::new(tokens.clone());
        
        let first = parser.advance().unwrap();
        assert_eq!(tokens[0], *first);
        let second = parser.advance().unwrap();
        assert_eq!(tokens[1], *second);
        let third = parser.advance();

        assert_eq!(third, Err(Error::ParserErr));
        
    }
}
