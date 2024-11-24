use crate::lexer::{Token,TokenType, Error};
use std::collections::HashMap;


// 定义 AST 节点
#[derive(Debug, Clone)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    Assignment{
        var_type: String,
        identifier: String,
        var_value: Option<Box<ASTNode>>,
    },
    FunctionDefinition{
        fn_name: String,
        parameters:Vec<(String, String)>,
        ret_type: Option<String>,
        body: Vec<ASTNode>
    },
    FunctionCall{
        fn_name:String,
        argument: Vec<(String, String)>,
    },
    InfixExpression{
        left_expr: Box<ASTNode>,
        op: String,
        right_expr:Box<ASTNode>,
    },
    Return(Box<ASTNode>),
    Expression(Box<ASTNode>),
    Number(String),
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

    fn expect(&mut self, ty:TokenType, value: String) -> Result<(),String>{
        let c = self.advance().unwrap();
        if c.token_type == ty && c.value == value {
            Ok(())
        }else{
            Err(format!("Expected '{:?}' {:?}, found '{:?}' {:?}", ty, value, c.token_type, c.value))
        }
    }

    fn parse_program(&mut self) -> Result<ASTNode, String>{
        let mut statements:Vec<ASTNode> = Vec::new();
        while self.peek().unwrap().token_type != TokenType::EOF{
            statements.push(self.parse_statement()?);
        }
        Ok(ASTNode::Program(statements))

    }

    
    fn parse_statement(&mut self) -> Result<ASTNode, String>{
        let token = self.peek().unwrap();
        match token.token_type{
            TokenType::KEYWORD => {
                match token.value.as_str() {
                    "sub" => self.parse_function_definition(),
                    "ret" => self.parse_return(),
                    "i32" => self.parse_assignment(),
                    _ => Err(format!("parse_statement error"))
                }
            }
            _ => self.parse_expression()

        }
    }


    fn parse_function_definition(&mut self) -> Result<ASTNode, String>{
        self.expect(TokenType::KEYWORD, String::from("sub"))?;
        let fn_name = handle_identifier(self.advance().unwrap().value.as_str())?;
        self.expect(TokenType::LPAREN, String::from("("))?;
        let mut parameters = Vec::new();
        while self.peek().unwrap().token_type != TokenType::RPAREN{  
            let para_type = handle_type(self.advance().unwrap().value.as_str())?;
            let para_name = handle_identifier(self.advance().unwrap().value.as_str())?;
            parameters.push((para_type, para_name));
            if self.peek().unwrap().token_type == TokenType::COMMA{
                self.advance();
            }
        }
        self.expect(TokenType::RPAREN, String::from(")"))?;
        let ret_type = if self.peek().unwrap().token_type == TokenType::ARROW{
            self.advance();
            Some(self.advance().unwrap().value.clone())
        }else{
            None
        };
        let body = self.parse_block()?;
        Ok(ASTNode::FunctionDefinition { 
            fn_name, 
            parameters, 
            ret_type, 
            body ,
        })
    }

    fn parse_return(&mut self) -> Result<ASTNode, String>{
        self.expect(TokenType::KEYWORD, String::from("ret"))?;
        let value = self.parse_statement()?;
        Ok(ASTNode::Return(Box::new(value)))

    }

    fn parse_assignment(&mut self) -> Result<ASTNode, String>{
        let var_type = handle_type(self.advance().unwrap().value.as_str())?;
        let identifier = handle_identifier(self.advance().unwrap().value.as_str())?;
        let var_value = if self.peek().unwrap().token_type == TokenType::EQUALS{
            self.advance();
            Some(Box::new(self.parse_expression()?))
        } else{
            None
        };
        self.expect(TokenType::SEMICOLON, String::from(";"))?;
        Ok(ASTNode::Assignment { 
            var_type, 
            identifier, 
            var_value,
        })
    }

    fn parse_expression(&mut self) -> Result<ASTNode, String>{
        let left_expr = if self.peek().unwrap().token_type == TokenType::NUMBER {
            ASTNode::Number(self.advance().unwrap().value.clone())
        } else if self.peek().unwrap().token_type == TokenType::ID {
            ASTNode::Identifier(self.advance().unwrap().value.clone())
        }else {
            return Err(format!("Unexpected token: {:?}", self.peek()))
        };

       
        if ["+","-","*","/"].contains(&self.peek().unwrap().value.as_str()){
            let op = self.advance().unwrap().value.clone();
            let right_expr = self.parse_expression()?;
            return Ok(ASTNode::InfixExpression { 
                left_expr:Box::new(left_expr),
                op, 
                right_expr:Box::new(right_expr),
            });
        }

        Ok(left_expr)

    }

    fn parse_block(&mut self) -> Result<Vec<ASTNode>, String>{
        self.expect(TokenType::LBRACE, String::from("{"))?;
        let mut statements = Vec::new();
        while self.peek().unwrap().token_type != TokenType::RBRACE{
            statements.push(self.parse_statement()?);
        }
        self.expect(TokenType::RBRACE, String::from("}"));
        Ok(statements)

    }
}


fn handle_identifier(ident: &str) -> Result<String, String>{
    let keywords = ["i32", "str", "ret", "sub", "if","else", "while", "for", "in", "call"];
    if keywords.contains(&ident){
        Err(format!("{ident} is a keyword. Cannot use keyword as identifier."))
    }else{
        Ok(ident.to_string())
    }

}

fn handle_type(ty: &str) -> Result<String, String>{
    let keywords = ["i32"];
    if keywords.contains(&ty){
        Err(format!("{ty} is not a valid type."))
    }else{
        Ok(ty.to_string())
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

    #[test]
    fn expect(){
        let list = [
            Token{
                token_type: TokenType::KEYWORD,
                value:String::from("i32"),    
            },
            Token{
                token_type: TokenType::KEYWORD,
                value:String::from("sub"),    
            }
        ];
        let tokens = Vec::from(list);
        let mut parser = Parser::new(tokens.clone());
        assert!(parser.expect(TokenType::KEYWORD, String::from("i32")).is_ok());
        assert!(parser.expect(TokenType::KEYWORD,String::from("sub")).is_ok());

        

    }
}
