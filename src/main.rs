use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug,PartialEq)]
enum TokenType{
    KEYWORD,
    ID,
    NUMBER,
    CHAR,
    STRING,
    EQUALS,
    COLON,
    SEMICOLON,
    LBRACE,
    RBRACE,
    LPAREN,
    RPAREN,
    QUOTES,
    DQUOTES,
    EOF,
}

#[derive(Debug)]
enum Error{

}

#[derive(Debug,PartialEq)]
struct Token{
    token_type: TokenType,
    value: String,
}

struct LEXER<'a>{
    src: Peekable<Chars<'a>>
}

impl<'a> LEXER<'a>{
    fn new(src: &'a str) -> Self {
        Self{src:  src.chars().peekable()}
    }

    fn next_token(&mut self) -> Option<Token>{
        while let Some(&c) = self.src.peek(){
            
            if c.is_whitespace() {self.src.next();}
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
            || value == String::from("return")
            || value == String::from("sub")
        {
            Token{
                token_type: TokenType::KEYWORD,
                value: value,
            }
        } else {
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

    fn collect_symbol(&mut self)->Token{
        match self.src.next(){
            Some('=') => Token{token_type: TokenType::EQUALS, value:String::from("=")},
            Some(';') => Token{token_type: TokenType::SEMICOLON, value:String::from(";")},
            Some(':') => Token{token_type: TokenType::COLON, value:String::from(":")},
            Some('{') => Token{token_type: TokenType::LBRACE, value:String::from("{")},
            Some('}') => Token{token_type: TokenType::RBRACE, value:String::from("}")},
            Some('(') => Token{token_type: TokenType::LPAREN, value:String::from("(")},
            Some(')') => Token{token_type: TokenType::RPAREN, value:String::from(")")},
            Some('\'') => Token{token_type: TokenType::QUOTES, value:String::from("'")},
            Some('"') => Token{token_type: TokenType::DQUOTES, value:String::from("\"")},
            _ => Token{token_type:TokenType::EOF, value: String::from("")}

        }

    }

    

}

fn tokenization(lexer: &mut LEXER) -> Result<Vec<Token>, Error>{

    let mut tokens:Vec<Token> = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    Ok(tokens)
}

fn main(){

    let input = "i32 _a = 1;\nsub main{\n\treturn 0;\n}";
    println!("{input}");

    let mut lexer = LEXER::new(input);
    let mut tokens = Vec::new();

    tokens = tokenization(&mut lexer).unwrap();
    println!("{:?}", tokens);

}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn input_to_token(){
        let input = "i32 a = 1;\nsub main{\nreturn 0;}";
       
    
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
            Token { token_type: TokenType::KEYWORD, value: String::from("return") },
            Token { token_type: TokenType::NUMBER, value: String::from("0") },
            Token { token_type: TokenType::SEMICOLON, value: String::from(";") },
            Token { token_type: TokenType::RBRACE, value: String::from("}") },
        ];     

        assert_eq!(tokens, right_result);

        tokenization( &mut lexer);
    }
}