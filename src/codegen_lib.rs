pub fn generate_lib() -> String{
    let echo = generate_echo();
    format!("declare i32 @write(i32, i8*, i32) nounwind\n {echo}")

  
}


pub fn generate_echo() ->String{
    format!("define i32 @echo(i8* %__str, i32 %__len) {{\nentry:\n%__stdout_fd = alloca i32, align 4\nstore i32 1, i32* %__stdout_fd, align 4\ncall i32 @write(i32 1, i8* %__str, i32 %__len)\nret i32 0\n}}\n")
}


// pub fn generate_code_echo(str:String) -> String{
//     let len = str.len();
//     format!(" %str = alloca [6 x i8], align 1
//     store [6 x i8] c"hello\00", [6 x i8]* %str, align 1
//     %len = add i32 6, 0
//     call void @__echo(i8* %str, i32 %len)")
// }
