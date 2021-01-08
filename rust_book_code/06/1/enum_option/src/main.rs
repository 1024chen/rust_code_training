fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    //如果使用 None 而不是 Some，需要告诉 Rust Option 是什么类型的，
    //因为编译器只通过 None 值无法推断出 Some 成员保存的值的类型
    let absent_number:Option<i32> = None;
}
