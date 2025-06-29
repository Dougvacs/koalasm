@.str = private unnamed_addr constant [15 x i8] c"Hello, world!\0A\00", align 1

declare i32 @puts(ptr)

define i32 @main() {
entry:
  %msg = getelementptr [14 x i8], ptr @.str, i32 0, i32 0
  call i32 @puts(ptr %msg)
  ret i32 0
}
