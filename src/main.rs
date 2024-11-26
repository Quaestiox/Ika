mod lexer;
mod parser;
mod sema;
mod io;

use lexer::{LEXER, tokenization,Token,TokenType};
use parser::{Parser};
use sema::{SymbolTable};
use io::{read_fs};


fn main(){

    let args:Vec<String> = std::env::args().collect();
    let path = &args[1];

    let content = read_fs(path);
    let input = content.as_str();
    
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
