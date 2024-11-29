@0 = global i32 1
define i32 @add(i32 %a, i32 %b) {
	%3 = alloca i32
	store i32 %a, ptr %3
	%5 = alloca i32
	store i32 %b, ptr %5
	%7 = load i32, ptr %3
	%8 = load i32, ptr %5
	%9 = add i32 %7, %8
	%10 = alloca i32
	store i32 %9, ptr %10
	%11 = load i32, ptr %10
	ret i32 %11
}
define void @main() {
	%13 = alloca i32
	%14 = load i32, ptr @0
	store i32 14, ptr %13
	%15 = alloca i32
	%16 = alloca i32
	store i32 2, ptr %16
	%17 = load i32, ptr %16
	store i32 17, ptr %15
	%18 = alloca i32
	%19 = load i32, ptr %13
	%20 = load i32, ptr %15
	%21 = call i32 @add(i32 %19,i32 %20)
	%22 = alloca i32 
	store i32 %21, ptr %22 
	%23 = load i32, ptr %22
	store i32 23, ptr %18
ret void
}
