
declare i32 @echo(i8* , i32 ) 

@int_to_string.result =  global [12 x i8] zeroinitializer, align 1

define i8* @itos(i32  %0) {
  %2 = alloca i32, align 4
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  %5 = alloca i32, align 4
  %6 = alloca i32, align 4
  %7 = alloca i8, align 1
  store i32 %0, ptr %2, align 4
  store i32 0, ptr %3, align 4
  store i32 0, ptr %4, align 4
  %8 = load i32, ptr %2, align 4
  %9 = icmp slt i32 %8, 0
  br i1 %9, label %10, label %13

10:                                               ; preds = %1
  store i32 1, ptr %3, align 4
  %11 = load i32, ptr %2, align 4
  %12 = sub nsw i32 0, %11
  store i32 %12, ptr %2, align 4
  br label %13

13:                                               ; preds = %10, %1
  %14 = load i32, ptr %2, align 4
  %15 = icmp eq i32 %14, 0
  br i1 %15, label %16, label %21

16:                                               ; preds = %13
  %17 = load i32, ptr %4, align 4
  %18 = add nsw i32 %17, 1
  store i32 %18, ptr %4, align 4
  %19 = sext i32 %17 to i64
  %20 = getelementptr inbounds [12 x i8], ptr @int_to_string.result, i64 0, i64 %19
  store i8 48, ptr %20, align 1
  br label %37

21:                                               ; preds = %13
  br label %22

22:                                               ; preds = %25, %21
  %23 = load i32, ptr %2, align 4
  %24 = icmp sgt i32 %23, 0
  br i1 %24, label %25, label %36

25:                                               ; preds = %22
  %26 = load i32, ptr %2, align 4
  %27 = srem i32 %26, 10
  %28 = add nsw i32 %27, 48
  %29 = trunc i32 %28 to i8
  %30 = load i32, ptr %4, align 4
  %31 = add nsw i32 %30, 1
  store i32 %31, ptr %4, align 4
  %32 = sext i32 %30 to i64
  %33 = getelementptr inbounds [12 x i8], ptr @int_to_string.result, i64 0, i64 %32
  store i8 %29, ptr %33, align 1
  %34 = load i32, ptr %2, align 4
  %35 = sdiv i32 %34, 10
  store i32 %35, ptr %2, align 4
  br label %22

36:                                               ; preds = %22
  br label %37

37:                                               ; preds = %36, %16
  %38 = load i32, ptr %3, align 4
  %39 = icmp ne i32 %38, 0
  br i1 %39, label %40, label %45

40:                                               ; preds = %37
  %41 = load i32, ptr %4, align 4
  %42 = add nsw i32 %41, 1
  store i32 %42, ptr %4, align 4
  %43 = sext i32 %41 to i64
  %44 = getelementptr inbounds [12 x i8], ptr @int_to_string.result, i64 0, i64 %43
  store i8 45, ptr %44, align 1
  br label %45

45:                                               ; preds = %40, %37
  %46 = load i32, ptr %4, align 4
  %47 = sext i32 %46 to i64
  %48 = getelementptr inbounds [12 x i8], ptr @int_to_string.result, i64 0, i64 %47
  store i8 0, ptr %48, align 1
  store i32 0, ptr %5, align 4
  %49 = load i32, ptr %4, align 4
  %50 = sub nsw i32 %49, 1
  store i32 %50, ptr %6, align 4
  br label %51

51:                                               ; preds = %55, %45
  %52 = load i32, ptr %5, align 4
  %53 = load i32, ptr %6, align 4
  %54 = icmp slt i32 %52, %53
  br i1 %54, label %55, label %75

55:                                               ; preds = %51
  %56 = load i32, ptr %5, align 4
  %57 = sext i32 %56 to i64
  %58 = getelementptr inbounds [12 x i8], ptr @int_to_string.result, i64 0, i64 %57
  %59 = load i8, ptr %58, align 1
  store i8 %59, ptr %7, align 1
  %60 = load i32, ptr %6, align 4
  %61 = sext i32 %60 to i64
  %62 = getelementptr inbounds [12 x i8], ptr @int_to_string.result, i64 0, i64 %61
  %63 = load i8, ptr %62, align 1
  %64 = load i32, ptr %5, align 4
  %65 = sext i32 %64 to i64
  %66 = getelementptr inbounds [12 x i8], ptr @int_to_string.result, i64 0, i64 %65
  store i8 %63, ptr %66, align 1
  %67 = load i8, ptr %7, align 1
  %68 = load i32, ptr %6, align 4
  %69 = sext i32 %68 to i64
  %70 = getelementptr inbounds [12 x i8], ptr @int_to_string.result, i64 0, i64 %69
  store i8 %67, ptr %70, align 1
  %71 = load i32, ptr %5, align 4
  %72 = add nsw i32 %71, 1
  store i32 %72, ptr %5, align 4
  %73 = load i32, ptr %6, align 4
  %74 = add nsw i32 %73, -1
  store i32 %74, ptr %6, align 4
  br label %51

75:                                               ; preds = %51
  %__res = load i8*, i8* @int_to_string.result
  ret i8* %__res
}

define i32 @len(i8* %__input) {
  %t = alloca i8*
  store i8* %__input, i8* %t
  %2 = alloca ptr
  %3 = alloca i32, align 4
  store i8* %t, ptr %2
  store i32 0, ptr %3, align 4
  br label %4

4:                                                ; preds = %11, %1
  %5 = load ptr, ptr %2, align 8
  %6 = load i32, ptr %3, align 4
  %7 = sext i32 %6 to i64
  %8 = getelementptr inbounds i8, ptr %5, i64 %7
  %9 = load i8, ptr %8, align 1
  %10 = icmp ne i8 %9, 0
  br i1 %10, label %11, label %14

11:                                               ; preds = %4
  %12 = load i32, ptr %3, align 4
  %13 = add nsw i32 %12, 1
  store i32 %13, ptr %3, align 4
  br label %4

14:                                               ; preds = %4
  %15 = load i32, ptr %3, align 4
  ret i32 %15
}
