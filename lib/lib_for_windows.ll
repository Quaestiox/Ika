; declare i32 @write(i32, i8*, i32) nounwind

; define i32 @echo(i8* %__str, i32 %__len) {
; entry:
;     %__str_1 = alloca i8*
;     store i8* %__str , i8* %__str_1
;     %__stdout_fd = alloca i32, align 4
;     store i32 1, i32* %__stdout_fd, align 4
;     call i32 @write(i32 1, i8* %__str_1, i32 %__len)
;     ret i32 0
; }

declare i32 @WriteConsoleA(i32, i8*, i32, i32*, i8*) 
declare i32 @GetStdHandle(i32) 
; 常量：STD_OUTPUT_HANDLE


define i32 @echo(i8* %__str, i32 %__len) {
entry:
    ; 分配局部变量
    %__str_1 = alloca i8*
    store i8* %__str, i8* %__str_1

    ; 获取标准输出句柄 (stdout)
    %stdout_handle = alloca i32
    store i32 -11, i32* %stdout_handle
    %stdout = load i32, i32* %stdout_handle
    ; 分配一个变量存储写入的字节数
    %bytes_written = alloca i32, align 4
    store i32 0, i32* %bytes_written, align 4

    %hConsole = call i32 @GetStdHandle(i32 %stdout)


    ; 调用 WriteFile 输出数据到标准输出
    call i32 @WriteConsoleA(i32 %hConsole, i8* %__str_1, i32 %__len, i32* %bytes_written, i32 0)

    ; 返回 0，表示函数执行成功
    ret i32 0
}
