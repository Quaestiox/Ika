target triple = "x86_64-pc-windows-msvc"
declare i32 @write(i32, i8*, i32) nounwind
 define i32 @echo(i8* %__str, i32 %__len) {
entry:
%__stdout_fd = alloca i32, align 4
store i32 1, i32* %__stdout_fd, align 4
call i32 @write(i32 1, i8* %__str, i32 %__len)
ret i32 0
}
@0 = global  [8 x i8] c"Hello, \00"
define void @main() {
	%2 = alloca i8*
	%3 = alloca [6 x i8]
	store [6 x i8] c"World\00", [6 x i8]* %3
	%4 = load i8*, i8** %3
	store i8* %4, i8** %2
	%5 = alloca i32
	store i32 7, i32* %5
	%6 = load i32 , i32* %5
	%7 = call i32 @echo(i8* @0,i32 %6)
	%8 = alloca i32 
	store i32 %7, ptr %8 
	%9 = alloca i32
	store i32 5, i32* %9
	%10 = load i32 , i32* %9
	%11 = call i32 @echo(i8* %2,i32 %10)
	%12 = alloca i32 
	store i32 %11, ptr %12 
ret void
}
