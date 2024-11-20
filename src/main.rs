use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
enum TokenType{
    ID,
    NUMBER,
    EQUALS,
    COLON,
    SEMICOLON,
    EOF,
}

#[derive(Debug)]
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
            if c.is_whitespace(){ self.src.next();}
            else if c.is_alphabetic(){
                return Some(self.collect_identifier());
                
            }else if c.is_digit(10){
                return Some(self.collect_number());
            }else {
                return Some(self.collect_symbol())

            }
        }
        None
    }

    fn collect_identifier(&mut self)->Token{
        let value:String = self.src.by_ref().take_while(|c| c.is_alphanumeric()).collect();
        
        Token{
            token_type: TokenType::ID,
            value: value,
        }

    }

    fn collect_number(&mut self)->Token{
        let value:String = self.src.by_ref().take_while(|c| c.is_digit(10)).collect();
        
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
            _ => Token{token_type:TokenType::EOF, value: String::from("")}

        }

    }

    

}

fn tokenization(lexer: &mut LEXER){
    while let Some(token) = lexer.next_token() {
        println!("{:?}", token);
    }
}

fn main(){

    let input = "i32 a = 1;\nmain:\nreturn 0;";

    let mut lexer = LEXER::new(input);

    tokenization( &mut lexer);




}