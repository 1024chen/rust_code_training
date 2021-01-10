fn main() {
    // let 语句，不过模式被指定为可反驳模式 Some(x),编译器会报错
    //let Some(x) = some_option_value;

    //为了修复在需要不可反驳模式的地方使用可反驳模式的情况，可以修改使用模式的代码：不同于使用 let，可以使用 if let
    //如果模式不匹配，大括号中的代码将被忽略，其余代码保持有效
    let some_option_value:Option<i32> = None;
    if let Some(x) = some_option_value {
        println!("{}",x);
    }

    //Rust 会抱怨将不可反驳模式用于 if let 是没有意义的：
    // if let x = 5 {
    //     println!("{}", x);
    // };

    //match匹配分支必须使用可反驳模式，除了最后一个分支需要使用能匹配任何剩余值的不可反驳模式。
    //Rust允许我们在只有一个匹配分支的match中使用不可反驳模式，不过这么做不是特别有用，并可以被更简单的 let 语句替代
    
}
