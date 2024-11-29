use crate::parser::{ASTNode};
use crate::lexer::{TokenType};
use core::net;
use std::collections::HashMap;

pub struct Codegen {
    output: String,
    tmp: i64,
    scope: i32,
    variables: HashMap<String, VarInfo>, 
}

#[derive(Debug, Clone)]
pub struct VarInfo{
    tmp_name: String,
    ty:String,
    scope: i32,
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

            ASTNode::Assignment { 
                identifier, 
                var_value 
            } => self.generate_code_assignment(identifier, var_value),

            ASTNode::FunctionDefinition { 
                fn_name, 
                parameters, 
                ret_type, 
                body 
            } => self.generate_code_fundef(fn_name, parameters, ret_type, body),

            ASTNode::Return(ast) => self.generate_code_return(*ast),
            
            ASTNode::FunctionCall { fn_name, argument } => {self.generate_code_funcall(fn_name, argument);()},
            _ => ()
        }
    }

    pub fn generate_code_vardef(&mut self, var_type:String, identifier:String, var_value:Option<Box<ASTNode>>){
        
        let llvm_var_type = turn_to_llvm_type(var_type).unwrap();


        if self.scope != 1{
            let tmp = self.tmp;
            self.tmp += 1;
            self.output.push_str(&format!("\t%{tmp} = alloca {llvm_var_type}\n"));
           
            match var_value{
                Some(expr) => {
                    let value = self.generate_code_expression(*expr);
                    let tmp2 = self.tmp;
                    self.tmp += 1;
                    self.output.push_str(&format!("\t%{tmp2} = load i32, ptr {value}\n"));
                    self.output.push_str(&format!("\tstore {llvm_var_type} {tmp2}, ptr %{tmp}\n"));
                }
                None => ()
            }
            let varinfo = VarInfo{
                tmp_name: format!("%{tmp}"),
                ty:llvm_var_type,
                scope:2,
            };
            self.variables.insert(identifier, varinfo);
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
            let varinfo = VarInfo{
                tmp_name: format!("@{tmp}"),
                ty:llvm_var_type,
                scope:1,
            };
            self.variables.insert(identifier, varinfo);

        }

    }


    fn generate_code_expression(&mut self, ast:ASTNode) -> String{
        match ast{
            ASTNode::InfixExpression { 
                left_expr, 
                op, 
                right_expr 
            } => {


                let left = self.generate_code_expression(*left_expr);
                

                let right = self.generate_code_expression(*right_expr);

                if self.scope != 1{

                    let tmp_left = self.tmp;
                    self.tmp += 1;
                    self.output.push_str(format!("\t%{tmp_left} = load i32, ptr {left}\n").as_str());

                    let tmp_right = self.tmp;
                    self.tmp += 1;
                    self.output.push_str(format!("\t%{tmp_right} = load i32, ptr {right}\n").as_str());

                    let tmp_res = self.tmp;
                    self.tmp += 1;
                    if op == "+"{
                        self.output.push_str(format!("\t%{tmp_res} = add i32 %{tmp_left}, %{tmp_right}\n").as_str());
                    } else if op == "-"{
                        self.output.push_str(format!("\t%{tmp_res} = sub i32 %{tmp_left}, %{tmp_right}\n").as_str());
                    }else if op == "*"{
                        self.output.push_str(format!("\t%{tmp_res} = mul i32 %{tmp_left}, %{tmp_right}\n").as_str());
                    }else if op == "/"{
                        self.output.push_str(format!("\t%{tmp_res} = udiv i32 %{tmp_left}, %{tmp_right}\n").as_str());
                    }else{

                    }

                    let tmp_new = self.tmp;
                    self.tmp += 1;
                    self.output.push_str(format!("\t%{tmp_new} = alloca i32\n").as_str());
                    self.output.push_str(format!("\tstore i32 %{tmp_res}, ptr %{tmp_new}\n").as_str());
                    format!("%{tmp_new}")
                } else{
                    let tmp_new = if op == "+"{
                        turn_string_to_int(left) + turn_string_to_int(right)
                    } else if op == "-"{
                        turn_string_to_int(left) - turn_string_to_int(right)
                    }else if op == "*"{
                        turn_string_to_int(left) * turn_string_to_int(right)
                    }else if op == "/"{
                        turn_string_to_int(left) / turn_string_to_int(right)
                    }else{
                        0
                    };
                    format!("{tmp_new}")
                }
            },
            ASTNode::Number(num) => {
                let tmp = self.tmp;
                self.tmp += 1;
                if self.scope != 1{
                    self.output.push_str(&format!("\t%{tmp} = alloca i32\n"));
                    self.output.push_str(&format!("\tstore i32 {num}, ptr %{tmp}\n"));
                    format!("%{tmp}")
                } else{
                    format!("{num}")

                }
 
            },
            ASTNode::Identifier(id) => {
                self.variables.get(&id).cloned().unwrap().tmp_name
            },
            ASTNode::FunctionCall{ fn_name, argument } => self.generate_code_funcall(fn_name, argument),

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

        let mut gen = String::new();

        for (i, para) in parameters.iter().enumerate() {
            let tmp = self.tmp;
            self.tmp += 1;
            let llvm_para_type = turn_to_llvm_type(para.0.clone()).unwrap();
            let para_name = &para.1;
            if i > 0 {
                self.output.push_str(", ");
            }
           
            

            let ltmp = self.tmp;
            self.tmp += 1;
            gen.push_str(format!("\t%{ltmp} = alloca {llvm_para_type}\n").as_str());
            gen.push_str(format!("\tstore {llvm_para_type} %{para_name}, ptr %{ltmp}\n").as_str());
            self.output.push_str(&format!("{} %{}", llvm_para_type, para_name));
            let varinfo = VarInfo{
                tmp_name: format!("%{ltmp}"),
                ty: llvm_para_type.clone(),
                scope: 2,
                
            };
            self.variables.insert(para_name.clone(), varinfo);
        }
        self.output.push_str(") {\n");
        self.output.push_str(&gen);
        

        self.scope += 1;
        self.tmp+= 1;
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
        let fun_info = VarInfo{
            tmp_name: format!("@{fn_name}"),
            ty: llvm_ret_type,
            scope: 1,
        };
        self.variables.insert(format!("{fn_name}"), fun_info);

    }

    fn generate_code_return(&mut self, ast:ASTNode){
        let value = self.generate_code_expression(ast);
     
        
        // let ty = var.ty.clone();
        let tmp = self.tmp;
        self.tmp += 1;
        self.output.push_str(&format!("\t%{tmp} = load i32, ptr {value}\n"));
        self.output.push_str(&format!("\tret i32 %{tmp}\n"));
    }

    fn generate_code_assignment(&mut self,  identifier: String, var_value: Option<Box<ASTNode>>,){  
        let llvm_var_info = self.variables.get(&identifier).cloned().unwrap();
        let value = self.generate_code_expression(*var_value.unwrap());
        if llvm_var_info.scope != 1{
            let ty = llvm_var_info.ty;
            let var_name = llvm_var_info.tmp_name;
            self.output.push_str(format!("\tstore {ty} {value}, ptr {var_name}\n").as_str());
        }else{
            let ty = llvm_var_info.ty;
            let var_name = llvm_var_info.tmp_name;
            self.output.push_str(format!("\tstore {ty} {value}, ptr {var_name}\n").as_str());
        }
        
    }

    fn generate_code_funcall(&mut self, fn_name:String, argument: Vec<ASTNode>)->String{
      

        let fun = self.variables.get(&fn_name).unwrap();
        let ret_type = fun.ty.clone();
        let mut values = Vec::new();
        for i in argument{
            let v = self.generate_code_expression(i);
            let ptmp = self.tmp;
            self.tmp += 1;
            self.output.push_str(&format!("\t%{ptmp} = load i32, ptr {v}\n"));
            values.push(format!("%{ptmp}"));

            
        }
        let tmp = self.tmp;
        self.tmp += 1;
        self.output.push_str(&format!("\t%{tmp} = call {ret_type} @{fn_name}("));
        for i in &values{
            if i == values.last().unwrap() {
                self.output.push_str(&format!("i32 {i}"));
            }else{
                self.output.push_str(&format!("i32 {i},"));
            }
            
        }

        let tmp2 = self.tmp;
        self.tmp += 1;
       

        self.output.push_str(&format!(")\n"));
        self.output.push_str(&format!("\t%{tmp2} = alloca i32 \n"));
        self.output.push_str(&format!("\tstore i32 %{tmp}, ptr %{tmp2} \n"));
        format!("%{tmp2}")

    }
  

    
}


fn turn_to_llvm_type(ty: String) -> Result<String, String> {
    match ty.as_str() {
        "i32" => Ok("i32".to_string()),
        _ => Err(format!("Cannot turn type '{}' to LLVM type", ty)),
    }
}

fn turn_string_to_int(str: String) -> i32{
    str.parse::<i32>().unwrap()
}


