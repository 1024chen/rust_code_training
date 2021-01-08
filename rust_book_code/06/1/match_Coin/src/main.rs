#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let coin: Coin = Coin::Penny;
    println!("the value is {}", value_in_cents(coin));
}

/*
match 关键字后跟一个表达式，在这个例子中是 coin 的值。
这看起来非常像 if 使用的表达式，不过这里有一个非常大的区别：
对于 if，表达式必须返回一个布尔值，而这里它可以是任何类型的。
*/
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
    /*
    一个分支有两个部分：一个模式和一些代码。
    第一个分支的模式是值 Coin::Penny 而之后的 => 运算符将模式和将要运行的代码分开。
    这里的代码就仅仅是值 1。每一个分支之间使用逗号分隔。
    */
}
/*
当 match 表达式执行时，它将结果值按顺序与每一个分支的模式相比较。
如果模式匹配了这个值，这个模式相关联的代码将被执行。
如果模式并不匹配这个值，将继续执行下一个分支
个分支相关联的代码是一个表达式，而表达式的结果值将作为整个 match 表达式的返回值。
*/
