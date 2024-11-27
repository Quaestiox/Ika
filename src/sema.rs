use std::collections::HashMap;

pub struct SymbolTable{
    variables: HashMap<String, String>,
    functions: HashMap<String, Function>,
}

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