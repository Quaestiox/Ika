use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug,PartialEq,Clone, Copy)]
pub enum TokenType{
    KEYWORD,
    ID,
    NUMBER,
    CHAR,
    STRING,
    EQUALS,
    DEQUALS,
    LT,
    LE,
    ST,
    SE,
    ADD,
    MINUS,
    ASTERISK,
    COLON,
    SEMICOLON,
    LBRACE,
    RBRACE,
    LPAREN,
    RPAREN,
    QUOTES,
    EX,
    UNEQ,
    DQUOTES,
    SLASH,
    COMMA,
    ARROW,
    AT,
    EOF,
}

#[derive(Debug,PartialEq)]
pub enum Error{
    LexerErr,
    ParserErr,
}

#[derive(Debug,PartialEq,Clone)]
pub struct Token{
    pub token_type: TokenType,
    pub value: String,
}

pub struct LEXER<'a>{
    src: Peekable<Chars<'a>>
}

impl<'a> LEXER<'a>{
    pub fn new(src: &'a str) -> Self {
        Self{src:  src.chars().peekable()}
    }

    pub fn next_token(&mut self) -> Option<Token>{
        while let Some(&c) = self.src.peek(){
            
            if c.is_whitespace() {self.src.next();}
            else if c == '/'{
                self.src.next();
                if let Some(&next_c) = self.src.peek() {
                    if next_c == '/' {
                        while let Some(comment_char) = self.src.next() {
                            if comment_char == '\n' {
                                break;
                            }
                        }
                        continue; 
                    } else {
                        return Some(Token {
                            token_type: TokenType::SLASH,
                            value: String::from("/"),
                        });
                    }
                }
            }
            else if c.is_alphabetic() || c == '_'{
                return Some(self.collect_identifier_keyword());
                
            }else if c.is_digit(10){
                return Some(self.collect_number());
            }else {
                return Some(self.collect_symbol());

            }
        }
        None
    }

    fn collect_identifier_keyword(&mut self)->Token{
        let mut value = String::new();

        while let Some(&c) = self.src.peek(){
            if c.is_alphanumeric() || c == '_'{
                value.push(c);
                self.src.next();
            }else{
                break;
            }
        }

        if value == String::from("i32")
            || value == String::from("bool")
            || value == String::from("str")
            || value == String::from("ret")
            || value == String::from("sub")
            || value == String::from("if")
            || value == String::from("else")
            || value == String::from("elif")
            || value == String::from("while")
            || value == String::from("for")
            || value == String::from("in")
            || value == String::from("call")
            
            
        {
            Token{
                token_type: TokenType::KEYWORD,
                value: value,
            }
        } else if value == "true".to_string(){
            Token{
                token_type: TokenType::NUMBER,
                value: "1".to_string(),
            }
        } else if value == "false".to_string(){
            Token{
                token_type: TokenType::NUMBER,
                value: "0".to_string(),
            }
        }
        else {
            Token{
                token_type: TokenType::ID,
                value: value,
            }
        }
        
       

    }

    fn collect_number(&mut self)->Token{
        let mut value = String::new();
        while let Some(&c) = self.src.peek(){
            if c.is_digit(10){
                value.push(c);
                self.src.next();
            }else{
                break;
            }
        }
        
        Token{
            token_type: TokenType::NUMBER,
            value: value,
        }
    }

    fn collect_minus(&mut self) -> Token{
        let c = self.src.peek().unwrap();
        if *c == '>'{
            self.src.next();
            Token{
                token_type: TokenType::ARROW,
                value:String::from("->"),
            }
        }else{
            Token{token_type: TokenType::MINUS, value:String::from("-")}
        }
        
    }
    
    fn collect_string(&mut self)->Token{
        let mut value = String::new();
        while let Some(&c) = self.src.peek(){
            if c != '\"'{
                value.push(c);
                self.src.next();
            }else{
                self.src.next();
                break;
            }
        }
        
        
        Token{
            token_type: TokenType::STRING,
            value: value,
        }


    }

    fn collect_eq(&mut self)-> Token{
        let c = self.src.peek().unwrap();
        if *c == '='{
            self.src.next();
            Token{
                token_type: TokenType::DEQUALS,
                value:String::from("=="),
            }
        }else{
            Token{token_type: TokenType::EQUALS, value:String::from("=")}
        }
    }

    fn collect_lt(&mut self)-> Token{
        let c = self.src.peek().unwrap();
        if *c == '='{
            self.src.next();
            Token{
                token_type: TokenType::LE,
                value:String::from(">="),
            }
        }else{
            Token{token_type: TokenType::LT, value:String::from(">")}
        }
    }
    
    fn collect_st(&mut self)-> Token{
        let c = self.src.peek().unwrap();
        if *c == '='{
            self.src.next();
            Token{
                token_type: TokenType::SE,
                value:String::from("<="),
            }
        }else{
            Token{token_type: TokenType::ST, value:String::from("<")}
        }
    }

    fn collect_ex(&mut self)-> Token{
        let c = self.src.peek().unwrap();
        if *c == '='{
            self.src.next();
            Token{
                token_type: TokenType::UNEQ,
                value:String::from("!="),
            }
        }else{
            Token{token_type: TokenType::EX, value:String::from("!")}
        }
    }
    
    

    fn collect_symbol(&mut self)->Token{
        match self.src.next(){
            Some('=') => self.collect_eq(),
            Some('>') => self.collect_lt(),
            Some('<') => self.collect_st(),
            Some(';') => Token{token_type: TokenType::SEMICOLON, value:String::from(";")},
            Some(':') => Token{token_type: TokenType::COLON, value:String::from(":")},
            Some('{') => Token{token_type: TokenType::LBRACE, value:String::from("{")},
            Some('}') => Token{token_type: TokenType::RBRACE, value:String::from("}")},
            Some('(') => Token{token_type: TokenType::LPAREN, value:String::from("(")},
            Some(')') => Token{token_type: TokenType::RPAREN, value:String::from(")")},
            Some('\'') => Token{token_type: TokenType::QUOTES, value:String::from("'")},
            Some('"') => self.collect_string(),
            Some('+') => Token{token_type: TokenType::ADD, value:String::from("+")},
            Some('-') => self.collect_minus(),
            Some('*') => Token{token_type: TokenType::ASTERISK, value:String::from("*")},
            Some(',') => Token{token_type: TokenType::COMMA, value:String::from(",")},
            Some('@') => Token{token_type: TokenType::AT, value:String::from("@")},
            Some('!') => self.collect_ex(),
            _ => Token{token_type:TokenType::EOF, value: String::from("")}

        }

    }

    

}

pub fn tokenization(lexer: &mut LEXER) -> Result<Vec<Token>, Error>{

    let mut tokens:Vec<Token> = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    Ok(tokens)
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn input_to_token(){
        let input = "i32 a = 1;//this is a comment!\nsub main{\nret 0;}";
       
    
        let mut lexer = LEXER::new(input);
        let mut tokens:Vec<Token> = Vec::new();

        while let Some(ctk) = lexer.next_token(){
            tokens.push(ctk);
        }

        let right_result = vec![
            Token { token_type: TokenType::KEYWORD, value: String::from("i32") },
            Token { token_type: TokenType::ID, value: String::from("a") },
            Token { token_type: TokenType::EQUALS, value: String::from("=")},
            Token { token_type: TokenType::NUMBER, value: String::from("1") },
            Token { token_type: TokenType::SEMICOLON, value: String::from(";") },
            Token { token_type: TokenType::KEYWORD, value: String::from("sub")},
            Token { token_type: TokenType::ID, value: String::from("main") },
            Token { token_type: TokenType::LBRACE, value: String::from("{") },
            Token { token_type: TokenType::KEYWORD, value: String::from("ret") },
            Token { token_type: TokenType::NUMBER, value: String::from("0") },
            Token { token_type: TokenType::SEMICOLON, value: String::from(";") },
            Token { token_type: TokenType::RBRACE, value: String::from("}") },
        ];     

        assert_eq!(tokens, right_result);

        tokenization( &mut lexer);
    }

   
    #[test]
    fn comment(){
        let input = "i32 a = 1;//This is a comment!\n i32 b = 1;";
       
    
        let mut lexer = LEXER::new(input);
        let mut tokens:Vec<Token> = Vec::new();

        while let Some(ctk) = lexer.next_token(){
            tokens.push(ctk);
        }

        let right_result = vec![
            Token { token_type: TokenType::KEYWORD, value: String::from("i32") },
            Token { token_type: TokenType::ID, value: String::from("a") },
            Token { token_type: TokenType::EQUALS, value: String::from("=")},
            Token { token_type: TokenType::NUMBER, value: String::from("1") },
            Token { token_type: TokenType::SEMICOLON, value: String::from(";") },
            Token { token_type: TokenType::KEYWORD, value: String::from("i32") },
            Token { token_type: TokenType::ID, value: String::from("b") },
            Token { token_type: TokenType::EQUALS, value: String::from("=")},
            Token { token_type: TokenType::NUMBER, value: String::from("1") },
            Token { token_type: TokenType::SEMICOLON, value: String::from(";") },
           
        ];     

        assert_eq!(tokens, right_result);

        tokenization( &mut lexer);
        
    }

    #[test]
    fn calculate(){
        let input = "i32 a = 1 + 5 - 7 * 4 / 2;";
       
    
        let mut lexer = LEXER::new(input);
        let mut tokens:Vec<Token> = Vec::new();

        while let Some(ctk) = lexer.next_token(){
            tokens.push(ctk);
        }

        let right_result = vec![
            Token { token_type: TokenType::KEYWORD, value: String::from("i32") },
            Token { token_type: TokenType::ID, value: String::from("a") },
            Token { token_type: TokenType::EQUALS, value: String::from("=")},
            Token { token_type: TokenType::NUMBER, value: String::from("1") },
            Token { token_type: TokenType::ADD, value: String::from("+") },
            Token { token_type: TokenType::NUMBER, value: String::from("5") },
            Token { token_type: TokenType::MINUS, value: String::from("-") },
            Token { token_type: TokenType::NUMBER, value: String::from("7")},
            Token { token_type: TokenType::ASTERISK, value: String::from("*") },
            Token { token_type: TokenType::NUMBER, value: String::from("4") },
            Token { token_type: TokenType::SLASH, value: String::from("/") },
            Token { token_type: TokenType::NUMBER, value: String::from("2") },
            Token { token_type: TokenType::SEMICOLON, value: String::from(";") },
           
        ];     

        assert_eq!(tokens, right_result);

        tokenization( &mut lexer);
        

    }

    #[test]
    fn string(){
        let input = "str a = \"aaa\";";
       
    
        let mut lexer = LEXER::new(input);
        let mut tokens:Vec<Token> = Vec::new();

        while let Some(ctk) = lexer.next_token(){
            tokens.push(ctk);
        }

        let right_result = vec![
            Token { token_type: TokenType::KEYWORD, value: String::from("str") },
            Token { token_type: TokenType::ID, value: String::from("a") },
            Token { token_type: TokenType::EQUALS, value: String::from("=")},
            Token { token_type: TokenType::STRING, value: String::from("aaa")},
            Token { token_type: TokenType::SEMICOLON, value: String::from(";") },
           
        ];     

        assert_eq!(tokens, right_result);

        tokenization( &mut lexer);
    }
}