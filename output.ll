target triple = "x86_64-pc-windows-msvc"
define void @main() {
	%1 = alloca i32
	%2 = alloca [2 x i8]
	store [2 x i8] c"a\00", [2 x i8]* %2
	%3 = load i32, i32* %2
	store i32 3, i32* %1
ret void
}
