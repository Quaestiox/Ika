use crate::parser::{ASTNode};
use crate::lexer::{TokenType};
use core::net;
use std::collections::HashMap;

pub struct Codegen {
    output: String,
    tmp: i64,
    scope: i32,
    variables: HashMap<String, String>, // 存储当前作用域中的变量
}

impl Codegen {
    pub fn new() -> Self {
        Codegen {
            output: String::new(),
            tmp: 0,
            scope: 1,
            variables: HashMap::new(),
        }
    }


    pub fn generate_code(&mut self, ast:ASTNode) -> &String{
        self.generate_program(ast);
        &self.output 
    }

    pub fn generate_program(&mut self, ast:ASTNode) {
        if let ASTNode::Program(vec) = ast{
            for stat in vec{
                self.generate_statement(stat);
            }
            
        }
    }

    pub fn generate_statement(&mut self, stat:ASTNode){
        match stat{
            ASTNode::VariableDefinition { 
                var_type, 
                identifier, 
                var_value 
            } => self.generate_code_vardef(var_type, identifier, var_value),

            ASTNode::FunctionDefinition { 
                fn_name, 
                parameters, 
                ret_type, 
                body 
            } => self.generate_code_fundef(fn_name, parameters, ret_type, body),

            _ => ()
        }
    }

    pub fn generate_code_vardef(&mut self, var_type:String, identifier:String, var_value:Option<Box<ASTNode>>){
        
        let llvm_var_type = turn_to_llvm_type(var_type).unwrap();


        if self.scope != 1{
            let tmp = self.tmp;
            self.tmp += 1;
            self.output.push_str(&format!("%{tmp} = alloca {llvm_var_type}"));
           
            match var_value{
                Some(expr) => {
                    let value = self.generate_code_expression(*expr);
                    self.output.push_str(&format!("store {llvm_var_type} {value}, ptr {tmp}\n"));
                }
                None => ()
            }
            self.variables.insert(identifier, format!("%{tmp}"));
        } else{
            let tmp = self.tmp;
            self.tmp += 1;
            self.output.push_str(&format!("@{tmp} = global {llvm_var_type}"));
           
            match var_value{
                Some(expr) => {
                    let value = self.generate_code_expression(*expr);
                    self.output.push_str(&format!(" {value}\n"));
                }
                None => {
                    self.output.push_str(&format!(" 0\n"));
                }
            }
            self.variables.insert(identifier, format!("@{tmp}"));

        }

    }


    fn generate_code_expression(&mut self, ast:ASTNode) -> String{
        match ast{
            ASTNode::InfixExpression { 
                left_expr, 
                op, 
                right_expr 
            } => {


                self.generate_code_expression(*left_expr);
                let tmp = self.tmp;
                self.tmp += 1;
                self.output.push_str(&format!(" "));

                if self.scope != 1{
                    format!("%{tmp}")
                } else{
                    format!("@{tmp}")
                }
            },
            ASTNode::Number(num) => {
                let tmp = self.tmp;
                self.tmp += 1;
                if self.scope != 1{
                    self.output.push_str(&format!("%{tmp} , alloca i32\n"));
                    self.output.push_str(&format!("store i32 {num}, ptr %{tmp}\n"));
                    format!("%{tmp}")
                } else {
                   
                    format!("{num}")
                }
              
            },
            ASTNode::Identifier(id) => {
                self.variables.get(&id).cloned().unwrap_or("".to_string())
            }

            _ => "".to_string()


        }

    }


    pub fn generate_code_fundef(&mut self,fn_name:String, parameters:Vec<(String, String)>, ret_type:Option<String>, body:Vec<ASTNode>){
        let llvm_ret_type = match ret_type {
            Some(ty) => turn_to_llvm_type(ty).unwrap(),
            None => "void".to_string(),
        };
        self.output.push_str(&format!(
            "define {} @{}(",
            llvm_ret_type, fn_name
        ));

        for (i, para) in parameters.iter().enumerate() {
            let llvm_para_type = turn_to_llvm_type(para.0.clone()).unwrap();
            let para_name = &para.1;
            if i > 0 {
                self.output.push_str(", ");
            }
            self.output.push_str(&format!("{} %{}", llvm_para_type, para_name));
            self.variables.insert(para_name.clone(), llvm_para_type);
        }
        self.output.push_str(") {\n");


        self.scope += 1;
        if llvm_ret_type == "void".to_string(){
            for stmt in body {
                self.generate_statement(stmt);
            }
            self.output.push_str("ret void\n");

        } else {
            for stmt in body {
                self.generate_statement(stmt);
            }
        }


        self.output.push_str("}\n");
        self.scope -= 1;
    }

   
    
    // fn change_var_to_tmp(var: String) -> Option<String>{

    // }

  

    
}


fn turn_to_llvm_type(ty: String) -> Result<String, String> {
    match ty.as_str() {
        "i32" => Ok("i32".to_string()),
        _ => Err(format!("Cannot turn type '{}' to LLVM type", ty)),
    }
}

