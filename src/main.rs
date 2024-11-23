mod lexer;
mod parser;

use lexer::{LEXER, tokenization};
use parser::{Parser};

fn main(){

    let input ="i32 a = 1 + 5 - 7 * 4 / 2;";
    println!("{input}");

    let mut lexer = LEXER::new(input);
    let mut tokens = Vec::new();

    


    tokens = tokenization(&mut lexer).unwrap();

    let mut parser = Parser::new(tokens.clone());

  
    println!("{:?}", tokens);

    

}
