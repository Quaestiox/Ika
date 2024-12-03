use crate::parser::{ASTNode};
use crate::lexer::{TokenType};
use crate::sema::Function;
use crate::SrcInfo;
use core::net;
use std::collections::HashMap;
use std::vec;
use crate::codegen_lib::{generate_lib};
pub struct Codegen {
    output: String,
    tmp: i64,
    scope: usize,
    pub sym_table: Vec<HashMap<String, Info>>, 
}

#[derive(Debug, Clone)]
pub enum Info{
    Variable{
        tmp_name:String,
        ty:String,
        scope: usize,
        size: i64,
    },
    Function{
        tmp_name:String,
        ret_ty: String,
        paras: Vec<String>,
        scope:usize,
    }
}



impl Codegen {
    pub fn new() -> Self {
        Codegen {
            output: String::new(),
            tmp: 0,
            scope: 1,
            sym_table: Vec::new(),
        }
    }

    fn new_tmp(&mut self) -> i64{
        let tmp = self.tmp;
        self.tmp += 1;
        tmp
    }

    fn add_to_symbol(&mut self, scope:usize, name:String, info:Info){
        
        let t = &mut self.sym_table;
        let a = t.get_mut(scope).unwrap();
        a.insert(name, info);
    }

    fn get_funinfo(&self, name: String) -> Option<(String, String, Vec<String>, usize)>{
        let fun =self.sym_table.get(1).clone().unwrap().get(&name).unwrap().clone();
        match fun {
            Info::Function { tmp_name, ret_ty, paras, scope } 
                => Some((tmp_name, ret_ty, paras, scope)),
            _ => None
        }
    }

    fn get_varinfo(&self, name:String)->Option<(String, String, usize, i64)>{
        let mut s =self.scope;
        while s >= 1{
            let a = self.sym_table.get(s ).clone().unwrap();
            let v = a.get(&name).cloned();
            if v.is_none(){
                s -= 1;
            }else{
                break;
            }
        }

        if s < 1{
            None
        }else{
            let var =self.sym_table.get(s).clone().unwrap().get(&name).cloned().unwrap().clone();
            match var {
                Info::Variable { tmp_name, ty, scope, size }
                   => Some((tmp_name, ty, scope, size)),
                _ => None
            }
        } 
    }

    pub fn generate_code(&mut self, ast:ASTNode, info:SrcInfo) -> &String{

        self.sym_table.push(HashMap::new());
        self.sym_table.push(HashMap::new());
        self.sym_table.push(HashMap::new());
        self.sym_table.push(HashMap::new());
        self.sym_table.push(HashMap::new());

        self.generate_program(ast, info);
        &self.output 
    }

    pub fn generate_program(&mut self, ast:ASTNode, info:SrcInfo) {
        
        let tt =info.target_triple;
        self.output.push_str(&format!("target triple = \"{tt}\"\n"));
        
        let v = &generate_lib();
        for i in v{
            self.output.push_str(i);
        }
        
        self.add_to_symbol(1, "echo".to_string(), Info::Function { tmp_name: "echo".to_string(), ret_ty: "i32".to_string(), paras:Vec::from(["i8*".to_string(),"i32".to_string()]), scope: 1 });
        self.add_to_symbol(1, "string".to_string(), Info::Function { tmp_name: "string".to_string(), ret_ty: "i8*".to_string(), paras:Vec::from(["i32*".to_string()]), scope: 1 });
      
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

            ASTNode::IfElse { condition, if_body, elif_body, else_body 
            } => self.generate_code_ifelse(condition, if_body, elif_body, else_body),
            
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
                    // if llvm_var_type != "i8*"{
                        let tmp2 = self.tmp;
                        self.tmp += 1;
                        self.output.push_str(&format!("\t%{tmp2} = load {llvm_var_type}, {llvm_var_type}* {value}\n"));
                        self.output.push_str(&format!("\tstore {llvm_var_type} %{tmp2}, {llvm_var_type}* %{tmp}\n"));
                    // }
                    // else{}
                }
                None => ()
            }
            let varinfo = Info::Variable { tmp_name: format!("%{tmp}"), ty:llvm_var_type, scope: self.scope, size: 32 };
            self.add_to_symbol(self.scope, identifier, varinfo);
        } else{
            let tmp = self.tmp;
            self.tmp += 1;
            if llvm_var_type != "i8*"{
                self.output.push_str(&format!("@{tmp} = global {llvm_var_type}"));
            } else{
                self.output.push_str(&format!("@{tmp} = global "));
            }
           
           
            match var_value{
                Some(expr) => {
                    let value = self.generate_code_expression(*expr);
                    self.output.push_str(&format!(" {value}\n"));
                }
                None => {
                    self.output.push_str(&format!(" 0\n"));
                }
            }
            let varinfo = Info::Variable { tmp_name: format!("@{tmp}"), ty:llvm_var_type, scope: 1, size: 32 };
            self.add_to_symbol(self.scope, identifier, varinfo);

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
                    self.output.push_str(format!("\t%{tmp_left} = load i32, i32* {left}\n").as_str());

                    let tmp_right = self.tmp;
                    self.tmp += 1;
                    self.output.push_str(format!("\t%{tmp_right} = load i32, i32* {right}\n").as_str());

                    let tmp_res = self.tmp;
                    self.tmp += 1;
                    let mut ty = String::new();
                    if op == "+"{
                        self.output.push_str(format!("\t%{tmp_res} = add i32 %{tmp_left}, %{tmp_right}\n").as_str());
                        ty = "i32".to_string();
                    } else if op == "-"{
                        self.output.push_str(format!("\t%{tmp_res} = sub i32 %{tmp_left}, %{tmp_right}\n").as_str());
                        ty = "i32".to_string();
                    }else if op == "*"{
                        self.output.push_str(format!("\t%{tmp_res} = mul i32 %{tmp_left}, %{tmp_right}\n").as_str());
                        ty = "i32".to_string();
                    }else if op == "/"{
                        self.output.push_str(format!("\t%{tmp_res} = udiv i32 %{tmp_left}, %{tmp_right}\n").as_str());
                        ty = "i32".to_string();
                    }else if op == "=="{
                        self.output.push_str(format!("\t%{tmp_res} = icmp eq i32 %{tmp_left}, %{tmp_right}\n").as_str());
                        ty = "i1".to_string();
                    } else{

                    }

                    let tmp_new = self.tmp;
                    self.tmp += 1;
                    self.output.push_str(format!("\t%{tmp_new} = alloca {ty}\n").as_str());
                    self.output.push_str(format!("\tstore {ty} %{tmp_res}, ptr %{tmp_new}\n").as_str());
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
                
                if self.scope != 1{
                    let tmp = self.tmp;
                    self.tmp += 1;
                    self.output.push_str(&format!("\t%{tmp} = alloca i32\n"));
                    self.output.push_str(&format!("\tstore i32 {num}, i32* %{tmp}\n"));
                    format!("%{tmp}")
                } else{
                    format!("{num}")

                }
 
            },
            ASTNode::String(value)=>{
                let len = value.len() + 1;
                if self.scope != 1{
                    let tmp = self.tmp;
                    self.tmp += 1;
                    self.output.push_str(&format!("\t%{tmp} = alloca [{len} x i8]\n"));
                    self.output.push_str(&format!("\tstore [{len} x i8] c\"{value}\\00\", [{len} x i8]* %{tmp}\n"));
                    format!("%{tmp}")
                } else{
                    format!("[{len} x i8] c\"{value}\\00\"")

                }
 

            }
            ASTNode::Identifier(id) => {
          
                self.get_varinfo(id).unwrap().0
                
                
            },
            ASTNode::FunctionCall{ fn_name, argument } => self.generate_code_funcall(fn_name, argument),

            _ => "".to_string()
        }

    }

    pub fn generate_code_fundef(&mut self,fn_name:String, parameters:Vec<(String, String)>, ret_type:Option<String>, body:Vec<ASTNode>){
        let llvm_ret_type = match ret_type {
            Some(ty) => {
                if ty == "str".to_string(){
                    "i8*".to_string()
                }else{
                    ty
                }
            }
            None => "void".to_string(),
        };
        self.output.push_str(&format!(
            "define {} @{}(",
            llvm_ret_type, fn_name
        ));

        let mut tylist = Vec::new();
        for p in parameters.clone(){
            tylist.push(p.0);
        }

        let mut gen = String::new();

        for (i, para) in parameters.iter().enumerate() {
            
            let llvm_para_type = turn_to_llvm_type(para.0.clone()).unwrap();
            let para_name = &para.1;
            if i > 0 {
                self.output.push_str(", ");
            }
           
            

            let ltmp = self.tmp;
            self.tmp += 1;
            gen.push_str(format!("\t%{ltmp} = alloca {llvm_para_type}\n").as_str());
            gen.push_str(format!("\tstore {llvm_para_type} %{para_name},ptr %{ltmp}\n").as_str());
            self.output.push_str(&format!("{} %{}", llvm_para_type, para_name));

           let varinfo = Info::Variable { tmp_name: format!("%{ltmp}"), ty:llvm_para_type.clone(), scope: 2, size: 32 };
            self.add_to_symbol(self.scope, para_name.clone(), varinfo);
        }

        self.output.push_str(") {\n");
        self.output.push_str("entry:\n");
        self.output.push_str(&gen);
        
        self.tmp += 1;
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
        let funinfo = Info::Function { tmp_name: fn_name.clone(), ret_ty: llvm_ret_type.clone(), paras: tylist.clone(), scope: 1 };
        self.add_to_symbol(self.scope, fn_name, funinfo);

    }

    fn generate_code_return(&mut self, ast:ASTNode){
        let value = self.generate_code_expression(ast);
     
        
        // let ty = var.ty.clone();
        let tmp = self.tmp;
        self.tmp += 1;
        self.output.push_str(&format!("\t%{tmp} = load i32, i32* {value}\n"));
        self.output.push_str(&format!("\tret i32 %{tmp}\n"));
    }

    fn generate_code_assignment(&mut self,  identifier: String, var_value: Option<Box<ASTNode>>,){  
        let var = self.get_varinfo(identifier).unwrap();
        let value = self.generate_code_expression(*var_value.unwrap());
        let tmp =self.new_tmp();
       
        if var.2 != 1{
            let ty = var.1;
            let var_name = var.0;
            self.output.push_str(format!("\t%{tmp} = load {ty}, ptr {value}\n").as_str());
            self.output.push_str(format!("\tstore {ty} {tmp}, ptr {var_name}\n").as_str());
        }else{
            let ty = var.1;
            let var_name = var.0;
            self.output.push_str(format!("\t%{tmp} = load {ty}, ptr {value}\n").as_str());
            self.output.push_str(format!("\tstore {ty} {tmp}, ptr {var_name}\n").as_str());
        }
        
    }

    fn generate_code_funcall(&mut self, fn_name:String, argument: Vec<ASTNode>)->String{
      

        let fun =self.get_funinfo(fn_name.clone()).unwrap();
        let tylist = &fun.2;
        let ret_type = fun.1.clone();
        let mut values = Vec::new();

        for i in 0..argument.len(){
            let ast = argument.get(i).unwrap().clone();
            let v = self.generate_code_expression(ast);
            let t = tylist.get(i).unwrap();

            if tylist[i] != "i8*"{
                let ptmp = self.tmp;
                self.tmp += 1;
                self.output.push_str(&format!("\t%{ptmp} = load {t} , {t}* {v}\n"));
                values.push(format!("%{ptmp}"));
            } else{
                values.push(format!("{v}"));
            }      
        }

        if ret_type == "void".to_string(){
            self.output.push_str(&format!("\tcall void @{fn_name}("));
            for i in 0..values.len(){
                let v = values.get(i).unwrap();
                let t = tylist.get(i).unwrap();
                if v == values.last().unwrap() {
                    self.output.push_str(&format!("{t} {v}"));
                }else{
                    self.output.push_str(&format!("{t} {v},"));
                }
                
            }
            let tmp2 = self.tmp;
            self.tmp += 1;
       

            self.output.push_str(&format!(")\n"));
            "".to_string()

        }else{
            let tmp = self.tmp;
            self.tmp += 1;
            self.output.push_str(&format!("\t%{tmp} = call {ret_type} @{fn_name}("));
            for i in 0..values.len(){
                let v = values.get(i).unwrap();
                let t = tylist.get(i).unwrap();
                if v == values.last().unwrap() {
                    self.output.push_str(&format!("{t} {v}"));
                }else{
                    self.output.push_str(&format!("{t} {v},"));
                }
                
            }
            let tmp2 = self.tmp;
        self.tmp += 1;
       
        self.output.push_str(&format!(")\n"));
        self.output.push_str(&format!("\t%{tmp2} = alloca {ret_type} \n"));
        self.output.push_str(&format!("\tstore {ret_type} %{tmp},ptr %{tmp2} \n"));
        format!("%{tmp2}")
        }

    } 

    fn generate_code_ifelse(&mut self,condition:Box<ASTNode>, if_body:Vec<ASTNode>, elif_body:Option<Vec<ASTNode>>, else_body:Option<Vec<ASTNode>>) {
        let res = self.generate_code_expression(*condition);
        let mut jmp = 0;
        
        let tmp = self.new_tmp();
        let tmp1 = self.new_tmp(); // if
        let mut tmp2 = 0;// else
        let tmp3 = self.new_tmp(); // final
        if else_body.is_some(){
            tmp2 = self.new_tmp(); 
            jmp = tmp2;
        } else{
            jmp = tmp3;
        }
        
        
       
      
        self.output.push_str(format!("\t%{tmp} = load i1, ptr {res}\n").as_str());
        self.output.push_str(format!("\tbr i1 %{tmp}, label %__{tmp1}, label %__{jmp}\n").as_str());
        self.output.push_str(format!("__{tmp1}:\n").as_str());
        for stat in if_body{
            self.generate_statement(stat);
        }
        self.output.push_str(format!("\tbr label %__{tmp3}\n").as_str());
        if else_body.is_some(){
            self.output.push_str(format!("__{tmp2}:\n").as_str());
        }
        

        match else_body{
            Some(v) =>{
                for stat in v{
                    self.generate_statement(stat);
                }
                self.output.push_str(format!("\tbr label %__{tmp3}\n").as_str());
            }
            None => (),
        }
       
        self.output.push_str(format!("__{tmp3}:\n").as_str());




    }
}


fn turn_to_llvm_type(ty: String) -> Result<String, String> {
    match ty.as_str() {
        "i32" => Ok("i32".to_string()),
        "str" => Ok("i8*".to_string()),
        "ptr" => Ok("ptr".to_string()),
        "void" => Ok("void".to_string()),
        _ => Err(format!("Cannot turn type '{}' to LLVM type", ty)),
    }
}

fn turn_string_to_int(str: String) -> i32{
    str.parse::<i32>().unwrap()
}


