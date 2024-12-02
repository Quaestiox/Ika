declare i32 @write(i32, i8*, i32) nounwind

define i32 @echo(i8* %__str, i32 %__len) {
entry:
    %__stdout_fd = alloca i32, align 4
    store i32 1, i32* %__stdout_fd, align 4
    call i32 @write(i32 1, i8* %__str, i32 %__len)
    ret i32 0
}
