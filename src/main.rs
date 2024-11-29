mod lexer;
mod parser;
mod sema;
mod io;
mod codegen;

use clap::{Parser as cp};
use std::process::{Command, exit};
use codegen::Codegen;
use lexer::{LEXER, tokenization,Token,TokenType};
use parser::{Parser};
use sema::{SymbolTable, SYMBOL_TABLES};
use io::{read_fs};

#[derive(cp)]
#[command(name = "ika")]
#[command(about = "A simple compiler for ika ")]
struct Cli {
    /// Print the AST
    #[arg(short = 'a', long = "ast")]
    show_ast: bool,

    ///Specify the output file name, the default is a.out
    #[arg(short, long)]
    output: Option<String>,

    ///Print the source code
    #[arg(short = 's', long = "source")]
    show_source: bool,

    /// Print tokens
    #[arg(short = 't', long = "tokens")]
    show_tokens: bool,

    ///ika file, like xxx.ika
    #[arg(required = true)]
    input: String,
}


fn main() {
   
    let cli = Cli::parse();

    let input_file = &cli.input;
    let output_file = cli.output.unwrap_or_else(|| "a.out".to_string());

    let content = read_fs(input_file);
    let input = content.as_str();
    
    if cli.show_source{
        println!("{input}");
    }
   
    
    let mut lexer = LEXER::new(input);
    let mut tokens = Vec::new();
    tokens = tokenization(&mut lexer).unwrap();
    
    
    tokens.push(Token {
        token_type: TokenType::EOF,
        value: String::new(),
    });
    
    if cli.show_tokens{
        println!("{:?}", tokens);
    }
    
    let mut parser = Parser::new(tokens.clone());
    let mut codegen = Codegen::new();
    
    match parser.parse_program() {
        Ok(ast) => {

            if cli.show_ast{
                println!("{:#?}", ast);
            }
            
            let out = codegen.generate_code(ast);

            std::fs::write("output.ll", out).expect("Unable to write file");

            let compile_status = Command::new("./tools/bin/clang")
                .arg("-o")
                .arg(&output_file)
                .arg("output.ll")
                .status()
                .expect("Failed to run clang");

            if compile_status.success() {
                println!("Compilation successful. Executable: {}", output_file);         
            } else {
                eprintln!("Error during compilation.");
                exit(1);
            }
        },
        Err(err) => eprintln!("Error: {}", err),
    }

}