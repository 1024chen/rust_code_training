///Rust 最常用的宏形式是 声明宏（declarative macros）。它们有时也被称为 “macros by example”、“macro_rules! 宏” 或者就是 “macros”。
///macro_rules! 实际上就过时（deprecated）了
#[macro_export]//#[macro_export] 注解说明宏应该是可用的， 如果没有该注解，这个宏不能被引入作用域
macro_rules! vec {
    //首先，一对括号包含了整个模式。接下来是美元符号（ $ ），后跟一对括号，捕获了符合括号内模式的值以用于替换后的代码。$() 内则是 $x:expr ，其匹配 Rust 的任意表达式，并将该表达式记作 $x
    //$() 之后的逗号说明一个可有可无的逗号分隔符可以出现在 $() 所匹配的代码之后。紧随逗号之后的 * 说明该模式匹配零个或更多个 * 之前的任何模式
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
//当以 vec![1, 2, 3]; 调用该宏时，替换该宏调用所生成的代码会是下面这样：
//let mut temp_vec = Vec::new();
//temp_vec.push(1);
//temp_vec.push(2);
//temp_vec.push(3);
//temp_vec

// 第二种形式的宏被称为 过程宏（procedural macros），因为它们更像函数（一种过程类型）
// 过程宏接收 Rust 代码作为输入，在这些代码上进行操作，然后产生另一些代码作为输出，而非像声明式宏那样匹配对应模式然后以另一部分代码替换当前代码
// 有三种类型的过程宏（自定义派生（derive），类属性和类函数），不过它们的工作方式都类似。
// 当创建过程宏时，其定义必须位于一种特殊类型的属于它们自己的 crate 中

//use proc_macro;

//#[some_attribute]
//pub fn some_name(input: TokenStream) -> TokenStream {
//}
