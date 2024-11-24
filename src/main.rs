mod lexer;
mod parser;

use lexer::{LEXER, tokenization,Token,TokenType};
use parser::{Parser};

fn main(){

    let input ="i32 a = 1 + 5 - 7 * 4 / 2;";
    println!("{input}");

    let mut lexer = LEXER::new(input);
    let mut tokens = Vec::new();

    


    tokens = tokenization(&mut lexer).unwrap();
    tokens.push(Token {
        token_type: TokenType::EOF,
        value: String::new(), 
    });
    

    let mut parser = Parser::new(tokens.clone());
    println!("{:?}", tokens);

    match parser.parse_program() {
        Ok(ast) => println!("{:#?}", ast),
        Err(err) => eprintln!("Error: {}", err),
    }
    

}
