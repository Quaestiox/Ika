@0 = global i32 1
define i32 @add(i32 %a, i32 %b) {
	%1 = alloca i32
	store i32 %a,i32* %1
	%2 = alloca i32
	store i32 %b,i32* %2
	%3 = load i32, i32* %1
	%4 = load i32, i32* %2
	%5 = add i32 %3, %4
	%6 = alloca i32
	store i32 %5, i32* %6
	%7 = load i32, i32* %6
	ret i32 %7
}
define void @echo(i32 %k) {
	%8 = alloca i32
	store i32 %k,i32* %8
ret void
}
define void @main() {
	%9 = alloca i32
	%10 = load i32, i32* @0
	store i32 10, i32* %9
	%11 = alloca i32
	%12 = alloca i32
	store i32 2, i32* %12
	%13 = load i32, i32* %12
	%14 = load i32, i32* %9
	%15 = add i32 %13, %14
	%16 = alloca i32
	store i32 %15, i32* %16
	%17 = load i32, i32* %16
	store i32 17, i32* %11
	%18 = alloca i32
	%19 = load i32, i32* %9
	%20 = load i32, i32* %11
	%21 = call i32 @add(i32 %19,i32 %20)
	%22 = alloca i32 
	store i32 %21, i32* %22 
	%23 = load i32, i32* %22
	store i32 23, i32* %18
	%24 = load i32, i32* %18
	call void @echo(i32 %24)
ret void
}
