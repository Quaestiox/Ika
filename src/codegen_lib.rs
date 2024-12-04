pub fn generate_lib() -> Vec<String>{
    Vec::from([
        format!("declare i32 @echo(i8*, i32) nounwind\n"),
        format!("declare i8* @itos(i32) nounwind\n"),
        format!("declare i32 @len(i8*) nounwind\n"),
       
        format!("@int_to_string.result =  external global [12 x i8] \n")
    ])
  
}


