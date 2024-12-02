pub fn generate_lib() -> Vec<String>{
    Vec::from([
        format!("declare i32 @echo(i8*, i32) nounwind\n"),
        format!("declare i8* @string(i32*) nounwind\n"),
    ])
  
}


