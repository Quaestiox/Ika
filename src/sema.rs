use core::arch;
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use crate::parser::{ASTNode};

#[derive(Debug,Clone)]
pub struct SymbolTable{
    variables: HashMap<String, String>,
    functions: HashMap<String, Function>,
}


#[derive(Debug,Clone)]
pub struct Function {
    pub fn_name: String,
    pub paras: Vec<(String, String)>,
    pub ret_type: Option<String>,
}

impl SymbolTable {
    pub fn new() -> Self{
        Self { variables: HashMap::new(), functions: HashMap::new() }
    }

    pub fn add_variable(&mut self, name: String, ty: String)  {
        self.variables.insert(name, ty);
    }

    pub fn add_function(&mut self, name: String, func: Function) {
        self.functions.insert(name, func);
    }

    pub fn lookup_variable(&self, name: &str) -> Option<String> {
        self.variables.get(name).cloned()
    }

    pub fn lookup_function(&self, name: &str) -> Option<&Function> {
        self.functions.get(name)
    }

    pub fn has_variable(&self, name:&str) -> bool {
        if self.variables.get(name).is_none(){
            false
        } else{
            true
        }
    }

    pub fn has_function(&self, name:&str) -> bool {
        if self.functions.get(name).is_none(){
            false
        } else{
            true
        }
    }
}

#[derive(Debug)]
pub struct ScopeManager{
    pub stack: Vec<SymbolTable>
}

impl ScopeManager{
    fn new() -> Self{
        let mut sm = Self { stack: Vec::new() };
        sm.push_scope();
        sm
    }

   
    pub fn push_scope(&mut self) {
        self.stack.push(SymbolTable::new());
    }

    pub fn pop_scope(&mut self) {
        self.stack.pop();
    }

    pub fn current_scope_mut(&mut self) -> &mut SymbolTable {
        self.stack.last_mut().unwrap()
    }

    pub fn current_scope(&self) -> &SymbolTable {
        self.stack.last().unwrap()
    }

    

    pub fn global_scope(&self) -> &SymbolTable {
        self.stack.first().unwrap()
    }
    pub fn global_scope_mut(&mut self) -> &mut SymbolTable {
        self.stack.first_mut().unwrap()
    }

    pub fn is_global_scope(&self) -> bool{
        if self.stack.len() == 1{
            true
        } else{
            false
        }
    }
}

lazy_static!{
  
    pub static ref SYMBOL_TABLES: Arc<Mutex<ScopeManager>> = Arc::new(Mutex::new(ScopeManager::new()));
    
}

pub fn lib_insert_symbol(){
    SYMBOL_TABLES.lock().unwrap().global_scope_mut().add_function("echo".to_string(),  Function {
        fn_name: "echo".to_string(),
        paras: Vec::from([("str".to_string(), "string".to_string()),( "i32".to_string(), "len".to_string())]),
        ret_type: None,
    });
    SYMBOL_TABLES.lock().unwrap().global_scope_mut().add_function("string".to_string(),  Function {
        fn_name: "string".to_string(),
        paras: Vec::from([( "i32*".to_string(), "num".to_string())]),
        ret_type: None,
    });
}

pub fn get_var(name: String)->String{
    let sym = SYMBOL_TABLES.lock().unwrap();
    let info = sym.global_scope().lookup_variable(name.as_str()).unwrap().clone();
    info
}

pub fn current_index()->usize{
    let sym = SYMBOL_TABLES.lock().unwrap();
    sym.stack.len()-1
    
}

pub fn insert_var(name:String, var_type:String){
    
    let mut sym = SYMBOL_TABLES.lock().unwrap();
    sym.current_scope_mut().add_variable(name.clone(), var_type.clone());
}

pub fn has_var(name: String, scope:&mut usize) -> bool{
    let sym = SYMBOL_TABLES.lock().unwrap();
    loop {
        let st = sym.stack[*scope].clone();
        let b = st.has_variable(name.as_str());
        if b{
            return true;
        }else if *scope == 0{
            break;
        }else {
            *scope -= 1;
        }
        
    }
    false
}


pub fn get_fun(name: String)->Function{
    let sym = SYMBOL_TABLES.lock().unwrap();
    let info = sym.global_scope().lookup_function(name.as_str()).unwrap().clone();
    let a = info.fn_name.clone();
    let b = info.paras.clone();
    let c = info.ret_type.clone();
    Function{
        fn_name: a,
        paras: b,
        ret_type:c
    } 
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn add_has_value(){
        let mut st = SymbolTable::new();
        st.add_variable("a".to_string(), "i32".to_string());
        let res1 = st.has_variable("a");
        let res2 = st.has_variable("b");

        assert_eq!(res1, true);
        assert_eq!(res2, false);
    }

    #[test]
    fn lookup(){
        let mut st = SymbolTable::new();
        st.add_variable("a".to_string(), "i32".to_string());
        let res1 = st.lookup_variable("a");
        let res2 = st.lookup_variable("b");
        assert_eq!(res1, Some("i32".to_string()));
        assert_eq!(res2, None); 
    }
}