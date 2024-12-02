mod lexer;
mod parser;
mod sema;
mod io;
mod codegen;
mod codegen_lib;

use clap::{Parser as cp};
use std::process::{Command, exit};
use std::env;
use codegen::Codegen;
use lexer::{LEXER, tokenization,Token,TokenType};
use parser::{Parser};
use sema::{lib_insert_symbol, SymbolTable, SYMBOL_TABLES};
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

pub struct SrcInfo{
   target_triple:String,
}

fn main() {
   
    let cli = Cli::parse();

    let os = env::consts::OS;
    
    let tt = match os {
        "windows" => "x86_64-pc-windows-msvc".to_string(),
        "linux" => "x86_64-unknown-linux-gnu".to_string(),
        "macos" => "x86_64-apple-darwin".to_string(),
        "freebsd" => "x86_64-unknown-freebsd".to_string(),
        _ => "x86_64-unknown-linux-gnu".to_string(), 
    };

    let outfile_name= match os {
        "windows" => "out.exe".to_string(),
        "linux" => "a.out".to_string(),
        _ => "a.out".to_string(),
    };
    
    let src_info = SrcInfo{
        target_triple: tt
    };


    
    let input_file = &cli.input;
    let output_file = cli.output.unwrap_or_else(|| outfile_name);

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
    println!("{:?}", tokens);
    let mut parser = Parser::new(tokens.clone());
    let mut codegen = Codegen::new();

    lib_insert_symbol();
    
    match parser.parse_program() {
        
        Ok(ast) => {

            if cli.show_ast{
                println!("{:#?}", ast);
            }
            
            let out = codegen.generate_code(ast, src_info).clone();
           

            std::fs::write(".\\target\\output.ll", out).expect("Unable to write file");

            let compile_status = Command::new("llvm-link")
                .arg(".\\lib\\base.ll")
                .arg(".\\lib\\lib_for_linux.ll") 
                .arg(".\\lib\\lib_for_windows.ll")
                .arg(".\\target\\output.ll")
                .arg("-o")
                .arg(".\\target\\linked.ll")
                .status()
                .expect("Failed to run llvm-link");

            if compile_status.success() {
                println!("Link successful.");         
            } else {
                eprintln!("link error.");
                exit(1);
            }

            let compile_status2 = Command::new("clang")
                .arg("-Wno-override-module")
                .arg("-o") 
                .arg(&output_file)
                .arg(".\\target\\linked.ll")
                .status()
                .expect("Failed to run clang");

            if compile_status2.success() {
                println!("Compilation successful. Executable: {}", output_file);         
            } else {
                eprintln!("Error during compilation.");
                exit(1);
            }
        },
        Err(err) => eprintln!("Error: {}", err),
    }
    
    

}