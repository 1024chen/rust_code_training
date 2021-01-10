use std::vec;
///这小节很重要，我都没完全学懂
///函数的类型是 fn （使用小写的 “f” ）以免与 Fn 闭包 trait 相混淆
///fn 被称为 函数指针（function pointer）
fn add_one(x: i32) -> i32 {
    x + 1
}

//不同于闭包，fn 是一个类型而不是一个 trait，所以直接指定 fn 作为参数而不是声明一个带有 Fn 作为 trait bound 的泛型参数
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
/**
不同于闭包，fn 是一个类型而不是一个 trait，所以直接指定 fn 作为参数而不是声明一个带有 Fn 作为 trait bound 的泛型参数。

函数指针实现了所有三个闭包 trait（Fn、FnMut 和 FnOnce），所以总是可以在调用期望闭包的函数时传递函数指针作为参数。
倾向于编写使用泛型和闭包 trait 的函数，这样它就能接受函数或闭包作为参数。
 */

/**
作为一个既可以使用内联定义的闭包又可以使用命名函数的例子，让我们看看一个 map 的应用。
使用 map 函数将一个数字 vector 转换为一个字符串 vector，就可以使用闭包
  */
#[allow(unused)]
fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    //另一个实用的模式暴露了元组结构体和元组结构体枚举成员的实现细节。这些项使用 () 作为初始化语法，这看起来就像函数调用，同时它们确实被实现为返回由参数构造的实例的函数。它们也被称为实现了闭包 trait 的函数指针
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

#[allow(unused)]
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
