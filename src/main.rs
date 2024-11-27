mod lexer;
mod parser;
mod sema;
mod io;

use lexer::{LEXER, tokenization,Token,TokenType};
use parser::{Parser};
use sema::{SymbolTable};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};


lazy_static!{
    pub static ref SYMBOL_TABLE: Arc<Mutex<SymbolTable>> = Arc::new(Mutex::new(SymbolTable::new()));
}

fn main(){

    let path = std::env::args();
    println!("{:?}", path);

    let input ="i32 a = 1 * 2 + (3 - 4) / 5;\na = 5;";
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
